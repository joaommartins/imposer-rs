use lopdf::{
    content::{Content, Operation},
    {Dictionary, Document, Object, ObjectId, Stream},
};

use crate::config::{BookletConfig, Dimensions, GridLayout, ImpositionLayout, Scale, SheetPages};
use crate::error::{BookletError, Result};
use crate::imposition::{calculate_page_order, calculate_page_order_with_signatures};

/// Calculate the grid layout (rows, cols) for n-up imposition
///
/// Returns (rows, cols) tuple where rows * cols = `pages_per_sheet`
/// Attempts to create a roughly square grid, preferring more columns than rows
/// for landscape orientation.
fn calculate_grid_layout(pages_per_sheet: usize) -> (usize, usize) {
    if pages_per_sheet == 0 {
        return (1, 1);
    }

    // Find the largest factor <= sqrt(n) for rows
    // Note: usize to f64 precision loss is acceptable for grid calculations
    #[expect(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let sqrt = (pages_per_sheet as f64).sqrt().floor() as usize;
    let mut rows = 1;

    for r in (1..=sqrt).rev() {
        if pages_per_sheet.is_multiple_of(r) {
            rows = r;
            break;
        }
    }

    let cols = pages_per_sheet / rows;

    // Prefer landscape: ensure cols >= rows
    if rows > cols {
        (cols, rows)
    } else {
        (rows, cols)
    }
}

/// Generate an n-up booklet with custom configuration
///
/// Takes a PDF as input and produces a new PDF with pages arranged in booklet order,
/// with n pages side-by-side on each output page.
///
/// # Arguments
///
/// * `input_pdf` - The input PDF file as a byte slice
/// * `config` - Configuration options for the booklet
///
/// # Returns
///
/// A `Vec<u8>` containing the generated booklet PDF
///
/// # Errors
///
/// Returns `BookletError::ParseError` if the input PDF cannot be parsed.
/// Returns `BookletError::GenerationError` if the output PDF cannot be generated.
#[expect(clippy::too_many_lines)]
pub fn generate_booklet_with_config(input_pdf: &[u8], config: &BookletConfig) -> Result<Vec<u8>> {
    // Parse the input PDF
    let doc = Document::load_mem(input_pdf).map_err(|e| BookletError::ParseError(e.to_string()))?;

    // Get pages
    let pages_map = doc.get_pages();
    let num_pages = pages_map.len();

    if num_pages == 0 {
        return Err(BookletError::InvalidConfig("PDF has no pages".to_string()));
    }

    // Get page IDs in order
    let mut page_list: Vec<(u32, ObjectId)> = pages_map.into_iter().collect();
    page_list.sort_by_key(|(page_num, _)| *page_num);

    // Get first page dimensions
    let first_page_id = page_list[0].1;
    let (source_page_width, source_page_height) = get_page_dimensions(&doc, first_page_id)?;

    // Calculate grid layout (rows × cols)
    let (grid_rows, grid_cols) = calculate_grid_layout(config.pages_per_sheet);

    // Determine output page dimensions based on grid layout
    // The goal is to fit the grid of source pages (assumed portrait) onto the output
    let (base_width, base_height) = config.page_size.dimensions();

    // Calculate aspect ratio of source page
    let source_aspect = source_page_height / source_page_width;

    // For the output page, we need to fit grid_cols × grid_rows of these pages
    // The output aspect ratio should be approximately:
    // (cols * source_width) / (rows * source_height) = cols / (rows * source_aspect)
    // Note: u32 to f32 precision loss is acceptable for reasonable grid sizes (max 1024)
    let grid_cols_u32 = u32::try_from(grid_cols).unwrap_or(1);
    let grid_rows_u32 = u32::try_from(grid_rows).unwrap_or(1);
    #[expect(clippy::cast_precision_loss)]
    let grid_aspect = (grid_cols_u32 as f32) / ((grid_rows_u32 as f32) * source_aspect);

    // Choose output orientation based on grid aspect ratio
    let (output_width, output_height) = if grid_aspect > 1.0 {
        // Grid is wider than tall -> landscape output
        if base_height > base_width {
            (base_height, base_width) // Rotate to landscape
        } else {
            (base_width, base_height)
        }
    } else {
        // Grid is taller than wide -> portrait output
        if base_width > base_height {
            (base_height, base_width) // Rotate to portrait
        } else {
            (base_width, base_height)
        }
    };

    // Calculate how much space each page slot gets
    #[expect(clippy::cast_precision_loss)]
    let page_slot_width = output_width / (grid_cols_u32 as f32);
    #[expect(clippy::cast_precision_loss)]
    let page_slot_height = output_height / (grid_rows_u32 as f32);

    // Calculate scaling if needed
    let (scale_x, scale_y) = if config.scale_to_fit {
        // Each source page should fit in its grid slot
        let available_width = page_slot_width;
        let available_height = page_slot_height;

        let scale_x = available_width / source_page_width;
        let scale_y = available_height / source_page_height;

        if config.preserve_aspect_ratio {
            // Uniform scaling to maintain aspect ratio
            let scale = scale_x.min(scale_y);
            (scale, scale)
        } else {
            // Non-uniform scaling to fill the available space
            (scale_x, scale_y)
        }
    } else {
        (1.0, 1.0)
    };

    // Calculate booklet page order
    let page_order = if let Some(num_sigs) = config.num_signatures {
        // Use signature count if specified (only for perfect bound)
        calculate_page_order_with_signatures(num_pages, config.pages_per_sheet, num_sigs)
    } else {
        // Use standard calculation based on binding type
        calculate_page_order(
            num_pages,
            config.pages_per_sheet,
            config.binding_type,
            config.sheets_per_signature,
        )
    };

    // Create output document
    let mut output_doc = Document::with_version("1.4");

    // Create catalog and pages tree
    let pages_id = output_doc.new_object_id();
    let mut pages_dict = Dictionary::new();
    pages_dict.set("Type", Object::Name(b"Page".to_vec()));
    pages_dict.set("Kids", Object::Array(Vec::new()));
    pages_dict.set("Count", Object::Integer(0));

    // Create catalog
    let catalog_id = output_doc.new_object_id();
    let mut catalog = Dictionary::new();
    catalog.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog.set("Pages", Object::Reference(pages_id));

    let _ = output_doc.objects.insert(catalog_id, Object::Dictionary(catalog));
    output_doc.trailer.set("Root", Object::Reference(catalog_id));

    // Build each imposed page
    let mut page_refs = Vec::new();
    let mut object_cache = std::collections::HashMap::new();

    for page_nums in page_order {
        let sheet_page_id = create_imposed_page_nup(
            &mut output_doc,
            &doc,
            SheetPages::new(&page_list, &page_nums),
            ImpositionLayout::new(
                GridLayout::new(grid_rows, grid_cols),
                Dimensions::new(page_slot_width, page_slot_height),
                Dimensions::new(output_width, output_height),
                Dimensions::new(source_page_width, source_page_height),
                Scale::new(scale_x, scale_y),
            ),
            pages_id,
            &mut object_cache,
            config,
        )?;
        page_refs.push(Object::Reference(sheet_page_id));
    }

    // Update pages dictionary with all page references
    pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
    pages_dict.set("Kids", Object::Array(page_refs.clone()));
    pages_dict.set(
        "Count",
        Object::Integer(i64::try_from(page_refs.len()).unwrap_or(i64::MAX)),
    );
    pages_dict.set(
        "MediaBox",
        Object::Array(vec![0.into(), 0.into(), output_width.into(), output_height.into()]),
    );

    let _ = output_doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    // Save to bytes
    let mut output = Vec::new();
    output_doc
        .save_to(&mut output)
        .map_err(|e| BookletError::GenerationError(e.to_string()))?;

    Ok(output)
}

/// Get the dimensions of a page (width, height)
fn get_page_dimensions(doc: &Document, page_id: ObjectId) -> Result<(f32, f32)> {
    // Get the page object
    let page_obj = doc
        .get_object(page_id)
        .map_err(|e| BookletError::ParseError(format!("Failed to get page object: {e}")))?;

    // Extract page dictionary
    let page_dict = page_obj
        .as_dict()
        .map_err(|e| BookletError::ParseError(format!("Page is not a dictionary: {e}")))?;

    // Get MediaBox (may be inherited from parent)
    let media_box = page_dict
        .get(b"MediaBox")
        .and_then(|obj| obj.as_array())
        .map_err(|e| BookletError::ParseError(format!("Invalid MediaBox: {e}")))?;

    if media_box.len() < 4 {
        return Err(BookletError::ParseError("MediaBox has fewer than 4 values".to_string()));
    }

    let x1 = media_box[0].as_float().unwrap_or(0.0);
    let y1 = media_box[1].as_float().unwrap_or(0.0);
    let x2 = media_box[2].as_float().unwrap_or(0.0);
    let y2 = media_box[3].as_float().unwrap_or(0.0);

    let width = x2 - x1;
    let height = y2 - y1;

    Ok((width, height))
}

/// Create a single imposed page with n source pages arranged in a grid
#[expect(clippy::too_many_lines)]
fn create_imposed_page_nup(
    output_doc: &mut Document,
    source_doc: &Document,
    sheet_pages: SheetPages,
    layout: ImpositionLayout,
    pages_id: ObjectId,
    cache: &mut std::collections::HashMap<ObjectId, ObjectId>,
    config: &BookletConfig,
) -> Result<ObjectId> {
    // Extract config values
    let draw_guides = config.draw_guides;
    let number_pages = config.number_pages;

    // Create resources dictionary for XObjects
    let mut resources = Dictionary::new();
    let mut xobjects = Dictionary::new();

    // Create content stream operations
    let mut operations = Vec::new();

    // Process each page in the grid (reading order: left-to-right, top-to-bottom)
    for (slot_index, &page_num) in sheet_pages.page_nums.iter().enumerate() {
        if page_num > 0 && page_num <= sheet_pages.all_pages.len() {
            // Calculate grid position (row, col) for this slot
            // Pages fill left-to-right, top-to-bottom
            let row = slot_index / layout.grid.cols;
            let col = slot_index % layout.grid.cols;

            let col_u32 = u32::try_from(col).unwrap_or(0);
            let row_u32 = u32::try_from(row).unwrap_or(0);
            #[expect(clippy::cast_precision_loss)]
            let slot_x = layout.slot.width * (col_u32 as f32);
            // PDF coordinates start at bottom-left, so we need to flip the row
            #[expect(clippy::cast_precision_loss)]
            let slot_y = layout.output.height - layout.slot.height * ((row_u32 + 1) as f32);

            // Add operations to place page with scaling and clipping
            operations.push(Operation::new("q", vec![])); // Save graphics state

            // Set clipping rectangle for this slot
            operations.push(Operation::new(
                "re",
                vec![
                    slot_x.into(),
                    slot_y.into(),
                    layout.slot.width.into(),
                    layout.slot.height.into(),
                ],
            ));
            operations.push(Operation::new("W", vec![])); // Clip
            operations.push(Operation::new("n", vec![])); // End path without stroking

            if number_pages {
                // Draw page number instead of content
                // Draw a border for the slot
                operations.push(Operation::new(
                    "re",
                    vec![
                        slot_x.into(),
                        slot_y.into(),
                        layout.slot.width.into(),
                        layout.slot.height.into(),
                    ],
                ));
                operations.push(Operation::new("S", vec![])); // Stroke the rectangle

                // Draw the page number in the center
                let page_num_str = format!("{page_num}");
                let font_size = (layout.slot.height / 3.0).min(layout.slot.width / 2.0).min(144.0);

                // Estimate text width (rough approximation: 0.6 * font_size per digit for bold font)
                let str_len_u32 = u32::try_from(page_num_str.len()).unwrap_or(0);
                #[expect(clippy::cast_precision_loss)]
                let text_width = (str_len_u32 as f32) * font_size * 0.6;

                let text_x = slot_x + layout.slot.width / 2.0 - text_width / 2.0;
                let text_y = slot_y + layout.slot.height / 2.0 - font_size / 3.0;

                operations.push(Operation::new("BT", vec![])); // Begin text
                operations.push(Operation::new("Tf", vec!["F1".into(), font_size.into()]));
                operations.push(Operation::new("Td", vec![text_x.into(), text_y.into()]));
                operations.push(Operation::new(
                    "Tj",
                    vec![Object::String(page_num_str.into_bytes(), lopdf::StringFormat::Literal)],
                ));
                operations.push(Operation::new("ET", vec![])); // End text
            } else {
                // Place actual page content
                let source_page_id = sheet_pages.all_pages[page_num - 1].1;
                if let Ok(xobj_id) = create_form_xobject(
                    output_doc,
                    source_doc,
                    source_page_id,
                    layout.source.width,
                    layout.source.height,
                    cache,
                ) {
                    let page_name = format!("Page{slot_index}");
                    xobjects.set(page_name.as_bytes().to_vec(), Object::Reference(xobj_id));

                    // Calculate the scaled page dimensions
                    let scaled_width = layout.source.width * layout.scale.x;
                    let scaled_height = layout.source.height * layout.scale.y;
                    // Normal orientation - no rotation for perfect binding
                    // Center the scaled page within the slot
                    let x_offset = slot_x + (layout.slot.width - scaled_width) / 2.0;
                    let y_offset = slot_y + (layout.slot.height - scaled_height) / 2.0;

                    // Apply transformation (scale and translate)
                    operations.push(Operation::new(
                        "cm",
                        vec![
                            layout.scale.x.into(),
                            0.into(),
                            0.into(),
                            layout.scale.y.into(),
                            x_offset.into(),
                            y_offset.into(),
                        ],
                    ));

                    operations.push(Operation::new("Do", vec![Object::Name(page_name.as_bytes().to_vec())]));
                }
            }

            operations.push(Operation::new("Q", vec![])); // Restore graphics state
        }
    }

    // Draw guide lines if requested (for debugging)
    if draw_guides {
        // Set thin line width and gray color for guides
        operations.push(Operation::new("q", vec![])); // Save graphics state
        operations.push(Operation::new("w", vec![0.5.into()])); // Line width 0.5 points
        operations.push(Operation::new("RG", vec![0.5.into(), 0.5.into(), 0.5.into()])); // Gray stroke color

        // Draw vertical fold lines between columns
        for col in 1..layout.grid.cols {
            let col_u32 = u32::try_from(col).unwrap_or(0);
            #[expect(clippy::cast_precision_loss)]
            let x = layout.slot.width * (col_u32 as f32);
            operations.push(Operation::new("m", vec![x.into(), 0.into()]));
            operations.push(Operation::new("l", vec![x.into(), layout.output.height.into()]));
        }

        // Draw horizontal fold lines between rows
        for row in 1..layout.grid.rows {
            let row_u32 = u32::try_from(row).unwrap_or(0);
            #[expect(clippy::cast_precision_loss)]
            let y = layout.slot.height * (row_u32 as f32);
            operations.push(Operation::new("m", vec![0.into(), y.into()]));
            operations.push(Operation::new("l", vec![layout.output.width.into(), y.into()]));
        }

        operations.push(Operation::new("S", vec![])); // Stroke all fold lines

        // Draw cut lines at edges (solid lines)
        // Top edge
        operations.push(Operation::new("m", vec![0.into(), layout.output.height.into()]));
        operations.push(Operation::new(
            "l",
            vec![layout.output.width.into(), layout.output.height.into()],
        ));
        // Bottom edge
        operations.push(Operation::new("m", vec![0.into(), 0.into()]));
        operations.push(Operation::new("l", vec![layout.output.width.into(), 0.into()]));
        // Left edge
        operations.push(Operation::new("m", vec![0.into(), 0.into()]));
        operations.push(Operation::new("l", vec![0.into(), layout.output.height.into()]));
        // Right edge
        operations.push(Operation::new("m", vec![layout.output.width.into(), 0.into()]));
        operations.push(Operation::new(
            "l",
            vec![layout.output.width.into(), layout.output.height.into()],
        ));

        operations.push(Operation::new("S", vec![])); // Stroke the cut lines

        operations.push(Operation::new("Q", vec![])); // Restore graphics state
    }

    // Set resources
    if !xobjects.is_empty() {
        resources.set("XObject", Object::Dictionary(xobjects));
    }

    // Add font resources if in number_pages mode
    if number_pages {
        let mut font_dict = Dictionary::new();
        let mut f1_dict = Dictionary::new();
        f1_dict.set("Type", Object::Name(b"Font".to_vec()));
        f1_dict.set("Subtype", Object::Name(b"Type1".to_vec()));
        f1_dict.set("BaseFont", Object::Name(b"Helvetica-Bold".to_vec()));
        font_dict.set(b"F1".to_vec(), Object::Dictionary(f1_dict));
        resources.set("Font", Object::Dictionary(font_dict));
    }

    // Encode content stream
    let content = Content { operations };
    let content_data = content
        .encode()
        .map_err(|e| BookletError::GenerationError(e.to_string()))?;

    let content_id = output_doc.add_object(Stream::new(Dictionary::new(), content_data));

    // Create page dictionary
    let mut page_dict = Dictionary::new();
    page_dict.set("Type", Object::Name(b"Page".to_vec()));
    page_dict.set("Parent", Object::Reference(pages_id));
    page_dict.set(
        "MediaBox",
        Object::Array(vec![
            0.into(),
            0.into(),
            layout.output.width.into(),
            layout.output.height.into(),
        ]),
    );
    page_dict.set("Contents", Object::Reference(content_id));
    page_dict.set("Resources", Object::Dictionary(resources));

    let sheet_page_id = output_doc.add_object(page_dict);

    Ok(sheet_page_id)
}

/// Create a Form `XObject` from a source page
fn create_form_xobject(
    output_doc: &mut Document,
    source_doc: &Document,
    source_page_id: ObjectId,
    width: f32,
    height: f32,
    cache: &mut std::collections::HashMap<ObjectId, ObjectId>,
) -> Result<ObjectId> {
    use lopdf::{Dictionary, Object, Stream};

    // Get the source page
    let page_obj = source_doc
        .get_object(source_page_id)
        .map_err(|e| BookletError::ParseError(format!("Failed to get source page: {e}")))?;

    let page_dict = page_obj
        .as_dict()
        .map_err(|_| BookletError::ParseError("Source page is not a dictionary".to_string()))?;

    // Get page contents
    let contents = page_dict
        .get(b"Contents")
        .map_err(|_| BookletError::ParseError("Page has no contents".to_string()))?;

    // Get the content stream data (decompressed)
    let content_data = match contents {
        Object::Reference(ref_id) => {
            let content_obj = source_doc
                .get_object(*ref_id)
                .map_err(|e| BookletError::ParseError(format!("Failed to get content object: {e}")))?;

            match content_obj {
                Object::Stream(stream) => {
                    // Decompress the stream to get the actual content
                    stream
                        .decompressed_content()
                        .map_err(|e| BookletError::ParseError(format!("Failed to decompress content: {e}")))?
                }
                _ => return Err(BookletError::ParseError("Content is not a stream".to_string())),
            }
        }
        Object::Array(refs) => {
            // Multiple content streams - concatenate them (decompressed)
            let mut combined = Vec::new();
            for obj in refs {
                if let Object::Reference(ref_id) = obj {
                    if let Ok(Object::Stream(stream)) = source_doc.get_object(*ref_id) {
                        if let Ok(decompressed) = stream.decompressed_content() {
                            combined.extend_from_slice(&decompressed);
                        }
                    }
                }
            }
            combined
        }
        _ => return Err(BookletError::ParseError("Unexpected content type".to_string())),
    };

    // Copy resources if they exist
    let resources = if let Ok(res) = page_dict.get(b"Resources") {
        copy_object_deep(source_doc, output_doc, res, cache)?
    } else {
        Object::Dictionary(Dictionary::new())
    };

    // Create Form XObject dictionary
    let mut xobj_dict = Dictionary::new();
    xobj_dict.set("Type", Object::Name(b"XObject".to_vec()));
    xobj_dict.set("Subtype", Object::Name(b"Form".to_vec()));
    xobj_dict.set(
        "BBox",
        Object::Array(vec![0.into(), 0.into(), width.into(), height.into()]),
    );
    xobj_dict.set("Resources", resources);

    let xobj_stream = Stream::new(xobj_dict, content_data);
    let xobj_id = output_doc.add_object(xobj_stream);

    Ok(xobj_id)
}

/// Deep copy an object and all its references from source to destination document
/// Uses a cache to avoid duplicating objects
fn copy_object_deep(
    source_doc: &Document,
    dest_doc: &mut Document,
    obj: &Object,
    cache: &mut std::collections::HashMap<ObjectId, ObjectId>,
) -> Result<Object> {
    use lopdf::{Dictionary, Object};

    match obj {
        Object::Reference(ref_id) => {
            // Check if we've already copied this object
            if let Some(&new_id) = cache.get(ref_id) {
                return Ok(Object::Reference(new_id));
            }

            // Get the referenced object from source
            if let Ok(referenced_obj) = source_doc.get_object(*ref_id) {
                // Recursively copy it
                let copied = copy_object_deep(source_doc, dest_doc, referenced_obj, cache)?;
                // Add to destination and return reference
                let new_id = dest_doc.add_object(copied);
                let _ = cache.insert(*ref_id, new_id);
                Ok(Object::Reference(new_id))
            } else {
                Ok(Object::Null)
            }
        }
        Object::Dictionary(dict) => {
            let mut new_dict = Dictionary::new();
            for (key, value) in dict {
                let copied_value = copy_object_deep(source_doc, dest_doc, value, cache)?;
                new_dict.set(key.clone(), copied_value);
            }
            Ok(Object::Dictionary(new_dict))
        }
        Object::Array(arr) => {
            let mut new_arr = Vec::new();
            for item in arr {
                new_arr.push(copy_object_deep(source_doc, dest_doc, item, cache)?);
            }
            Ok(Object::Array(new_arr))
        }
        Object::Stream(stream) => {
            let mut new_dict = Dictionary::new();
            for (key, value) in &stream.dict {
                let copied_value = copy_object_deep(source_doc, dest_doc, value, cache)?;
                new_dict.set(key.clone(), copied_value);
            }
            Ok(Object::Stream(Stream {
                dict: new_dict,
                content: stream.content.clone(),
                allows_compression: stream.allows_compression,
                start_position: None,
            }))
        }
        // For primitive types, just clone
        _ => Ok(obj.clone()),
    }
}
