/// Standard paper sizes for booklet output
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PageSize {
    /// A4 paper (210 × 297 mm) - Default
    #[default]
    A4,
    /// A3 paper (297 × 420 mm)
    A3,
    /// A5 paper (148 × 210 mm)
    A5,
    /// Letter paper (8.5 × 11 inches)
    Letter,
    /// Legal paper (8.5 × 14 inches)
    Legal,
    /// Tabloid paper (11 × 17 inches)
    Tabloid,
    /// Custom size in points (1 point = 1/72 inch)
    Custom(f32, f32),
}

impl PageSize {
    /// Get the dimensions in points (width, height)
    #[must_use]
    pub fn dimensions(&self) -> (f32, f32) {
        match self {
            // DIN A series (in points, 1mm = 2.83465 points)
            PageSize::A4 => (595.0, 842.0),  // 210 × 297 mm
            PageSize::A3 => (842.0, 1191.0), // 297 × 420 mm
            PageSize::A5 => (420.0, 595.0),  // 148 × 210 mm
            // US paper sizes (in points, 1 inch = 72 points)
            PageSize::Letter => (612.0, 792.0), // 8.5 × 11 inches
            PageSize::Legal => (612.0, 1008.0), // 8.5 × 14 inches
            PageSize::Tabloid => (792.0, 1224.0), // 11 × 17 inches
            PageSize::Custom(width, height) => (*width, *height),
        }
    }

    /// Parse a page size from a string
    ///
    /// # Note
    /// This method is provided for backwards compatibility.
    /// Use the `FromStr` trait implementation instead: `page_size_str.parse()?`
    #[must_use]
    #[expect(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        s.parse().ok()
    }
}

impl std::str::FromStr for PageSize {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a4" => Ok(PageSize::A4),
            "a3" => Ok(PageSize::A3),
            "a5" => Ok(PageSize::A5),
            "letter" => Ok(PageSize::Letter),
            "legal" => Ok(PageSize::Legal),
            "tabloid" => Ok(PageSize::Tabloid),
            _ => Err(()),
        }
    }
}

/// Binding type for booklet imposition
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BindingType {
    /// Saddle stitch binding (sheets folded in half and stapled in the middle)
    /// Uses alternating page order for proper reading sequence
    #[default]
    SaddleStitch,
    /// Perfect binding (pages glued at the spine)
    /// Uses sequential page order in signatures
    PerfectBound,
}

/// Check if a number is a power of 2 (bit manipulation trick)
/// Returns true only for 2, 4, 8, 16, 32, ... (powers of 2)
#[inline]
fn is_power_of_two(n: usize) -> bool {
    n > 0 && n.is_power_of_two()
}

/// Valid pages per sheet for saddle stitch binding
///
/// Saddle stitch binding works with geometric sequences: 2^n where n ≥ 1
/// This includes: 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, ...
///
/// The algorithm uses `sections_per_sheet = pages_per_sheet / 2` and grid calculations
/// that depend on power-of-2 layouts. Any power of 2 ≥ 2 is theoretically supported.
///
/// Common values:
/// - 2-up: 1 page per side (2 pages per sheet)
/// - 4-up: 2 pages per side (4 pages per sheet)
/// - 8-up: 4 pages per side (8 pages per sheet)
/// - 16-up: 8 pages per side (16 pages per sheet)
/// - 32-up: 16 pages per side (32 pages per sheet)
/// - 64-up: 32 pages per side (64 pages per sheet)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SaddleStitchPages {
    /// 2-up (1 page per side)
    Pages2,
    /// 4-up (2 pages per side)
    Pages4,
    /// 8-up (4 pages per side)
    Pages8,
    /// 16-up (8 pages per side)
    Pages16,
    /// 32-up (16 pages per side)
    Pages32,
    /// 64-up (32 pages per side)
    Pages64,
    /// Any larger power of 2 (128, 256, 512, 1024, ...)
    Other(usize),
}

impl SaddleStitchPages {
    /// Try to create from a numeric value
    ///
    /// Accepts any power of 2 ≥ 2, not just the predefined enum variants.
    /// This allows unlimited page-per-sheet values (128, 256, 512, 1024, ...).
    #[must_use]
    pub fn from_usize(value: usize) -> Option<Self> {
        // Accept any power of 2 >= 2
        if !is_power_of_two(value) || value < 2 {
            return None;
        }

        // Map to enum variant if it's a common value
        match value {
            2 => Some(SaddleStitchPages::Pages2),
            4 => Some(SaddleStitchPages::Pages4),
            8 => Some(SaddleStitchPages::Pages8),
            16 => Some(SaddleStitchPages::Pages16),
            32 => Some(SaddleStitchPages::Pages32),
            64 => Some(SaddleStitchPages::Pages64),
            // For any larger power of 2, return Other variant with the actual value
            // The actual value will come from the usize in the CLI/config
            _ if is_power_of_two(value) => Some(SaddleStitchPages::Other(value)),
            _ => None,
        }
    }

    /// Get common predefined values for display in help text
    #[must_use]
    pub fn common_values() -> &'static [usize] {
        &[2, 4, 8, 16, 32, 64]
    }

    /// Get a description of valid values (any power of 2)
    #[must_use]
    pub fn valid_values_description() -> &'static str {
        "any power of 2 (2, 4, 8, 16, 32, 64, 128, 256, ...)"
    }

    /// Get a formatted string showing common values and the pattern
    #[must_use]
    pub fn valid_values_string() -> String {
        format!(
            "{} (or any power of 2: 128, 256, 512, 1024, ...)",
            Self::common_values()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saddle_stitch_pages_from_usize_common_values() {
        // Test the common predefined variants
        assert_eq!(
            SaddleStitchPages::from_usize(2),
            Some(SaddleStitchPages::Pages2)
        );
        assert_eq!(
            SaddleStitchPages::from_usize(4),
            Some(SaddleStitchPages::Pages4)
        );
        assert_eq!(
            SaddleStitchPages::from_usize(8),
            Some(SaddleStitchPages::Pages8)
        );
        assert_eq!(
            SaddleStitchPages::from_usize(16),
            Some(SaddleStitchPages::Pages16)
        );
        assert_eq!(
            SaddleStitchPages::from_usize(32),
            Some(SaddleStitchPages::Pages32)
        );
        assert_eq!(
            SaddleStitchPages::from_usize(64),
            Some(SaddleStitchPages::Pages64)
        );
    }

    #[test]
    fn test_saddle_stitch_pages_from_usize_large_powers_of_two() {
        // Test that larger powers of 2 are also accepted
        assert_eq!(
            SaddleStitchPages::from_usize(128),
            Some(SaddleStitchPages::Other(128))
        );
        assert_eq!(
            SaddleStitchPages::from_usize(256),
            Some(SaddleStitchPages::Other(256))
        );
        assert_eq!(
            SaddleStitchPages::from_usize(512),
            Some(SaddleStitchPages::Other(512))
        );
        assert_eq!(
            SaddleStitchPages::from_usize(1024),
            Some(SaddleStitchPages::Other(1024))
        );
        assert_eq!(
            SaddleStitchPages::from_usize(65536),
            Some(SaddleStitchPages::Other(65536))
        );
    }

    #[test]
    fn test_saddle_stitch_pages_from_usize_invalid() {
        // Test that non-powers of 2 are rejected
        assert_eq!(SaddleStitchPages::from_usize(1), None); // 2^0 = 1, but we need >= 2
        assert_eq!(SaddleStitchPages::from_usize(3), None);
        assert_eq!(SaddleStitchPages::from_usize(5), None);
        assert_eq!(SaddleStitchPages::from_usize(6), None);
        assert_eq!(SaddleStitchPages::from_usize(7), None);
        assert_eq!(SaddleStitchPages::from_usize(12), None);
        assert_eq!(SaddleStitchPages::from_usize(100), None);
        assert_eq!(SaddleStitchPages::from_usize(127), None);
    }

    #[test]
    fn test_saddle_stitch_pages_common_values() {
        let common = SaddleStitchPages::common_values();
        assert_eq!(common, &[2, 4, 8, 16, 32, 64]);
    }

    #[test]
    fn test_saddle_stitch_pages_valid_values_description() {
        let desc = SaddleStitchPages::valid_values_description();
        assert!(desc.contains("any power of 2"));
    }

    #[test]
    fn test_saddle_stitch_pages_valid_values_string() {
        let valid_str = SaddleStitchPages::valid_values_string();
        // Should show common values and indicate larger values are supported
        assert!(valid_str.contains("2, 4, 8, 16, 32, 64"));
        assert!(valid_str.contains("128, 256, 512"));
    }

    #[test]
    fn test_is_power_of_two() {
        // Test powers of 2 (including 1, which is 2^0)
        assert!(is_power_of_two(1)); // 2^0 = 1 is technically a power of 2
        assert!(is_power_of_two(2)); // 2^1
        assert!(is_power_of_two(4)); // 2^2
        assert!(is_power_of_two(8)); // 2^3
        assert!(is_power_of_two(16));
        assert!(is_power_of_two(256));
        assert!(is_power_of_two(1024));

        // Test non-powers of 2
        assert!(!is_power_of_two(0));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(5));
        assert!(!is_power_of_two(6));
        assert!(!is_power_of_two(7));
        assert!(!is_power_of_two(12));
        assert!(!is_power_of_two(100));
    }
}

/// Configuration options for booklet generation
#[derive(Debug, Clone)]
#[expect(clippy::struct_excessive_bools)]
pub struct BookletConfig {
    /// Output page size for the booklet
    pub page_size: PageSize,
    /// Whether to scale pages to fit the output size
    pub scale_to_fit: bool,
    /// Whether to preserve aspect ratio when scaling (default: true)
    /// If true, uses uniform scaling to maintain aspect ratio
    /// If false, scales independently in x and y to fill the available space
    pub preserve_aspect_ratio: bool,
    /// Number of pages to place on each side of a sheet (default: 2 for 2-up booklets)
    /// For example: 2 means 2 pages per side (4 pages per sheet total)
    ///              3 means 3 pages per side (6 pages per sheet total)
    pub pages_per_sheet: usize,
    /// Whether to draw fold and cut guide lines for debugging (default: false)
    pub draw_guides: bool,
    /// Whether to show page numbers instead of actual content (default: false)
    /// Useful for visualizing the page order and imposition layout
    pub number_pages: bool,
    /// Binding type for the booklet (saddle stitch or perfect bound)
    pub binding_type: BindingType,
    /// Number of sheets per signature for perfect bound (default: 1)
    /// Only used for perfect bound binding. Sheets within a signature are nested together
    /// like saddle stitch, then signatures are stacked behind each other.
    /// For saddle stitch, this value is ignored.
    pub sheets_per_signature: usize,
    /// Number of signatures for perfect bound (optional override)
    /// If set, pages are evenly distributed across this many signatures.
    /// Takes precedence over `sheets_per_signature`.
    /// Only used for perfect bound binding.
    pub num_signatures: Option<usize>,
}

impl Default for BookletConfig {
    fn default() -> Self {
        Self {
            page_size: PageSize::default(),
            scale_to_fit: true,
            preserve_aspect_ratio: true,
            pages_per_sheet: 2,
            draw_guides: false,
            number_pages: false,
            binding_type: BindingType::SaddleStitch,
            sheets_per_signature: 1,
            num_signatures: None,
        }
    }
}

impl BookletConfig {
    /// Create a new configuration with the specified page size
    #[must_use]
    pub fn new(page_size: PageSize) -> Self {
        Self {
            page_size,
            scale_to_fit: true,
            preserve_aspect_ratio: true,
            pages_per_sheet: 2,
            draw_guides: false,
            number_pages: false,
            binding_type: BindingType::SaddleStitch,
            sheets_per_signature: 1,
            num_signatures: None,
        }
    }

    /// Set whether to scale pages to fit
    #[must_use]
    pub fn with_scale_to_fit(mut self, scale: bool) -> Self {
        self.scale_to_fit = scale;
        self
    }

    /// Set whether to preserve aspect ratio when scaling
    #[must_use]
    pub fn with_preserve_aspect_ratio(mut self, preserve: bool) -> Self {
        self.preserve_aspect_ratio = preserve;
        self
    }

    /// Set the number of pages per sheet side (e.g., 2 for 2-up, 3 for 3-up)
    #[must_use]
    pub fn with_pages_per_sheet(mut self, pages: usize) -> Self {
        self.pages_per_sheet = pages;
        self
    }

    /// Set whether to draw fold and cut guide lines
    #[must_use]
    pub fn with_draw_guides(mut self, draw: bool) -> Self {
        self.draw_guides = draw;
        self
    }

    /// Set whether to show page numbers instead of content
    #[must_use]
    pub fn with_number_pages(mut self, number: bool) -> Self {
        self.number_pages = number;
        self
    }

    /// Set the binding type (saddle stitch or perfect bound)
    #[must_use]
    pub fn with_binding_type(mut self, binding_type: BindingType) -> Self {
        self.binding_type = binding_type;
        self
    }

    /// Set the number of sheets per signature for perfect bound (default: 1)
    /// Only applies to perfect bound binding. Ignored for saddle stitch.
    #[must_use]
    pub fn with_sheets_per_signature(mut self, sheets: usize) -> Self {
        self.sheets_per_signature = sheets;
        self
    }

    /// Set the number of signatures for perfect bound
    /// If set, pages are evenly distributed across this many signatures.
    /// Takes precedence over `sheets_per_signature`.
    /// Only applies to perfect bound binding.
    #[must_use]
    pub fn with_num_signatures(mut self, num: Option<usize>) -> Self {
        self.num_signatures = num;
        self
    }
}

/// Grid layout dimensions (rows × columns)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridLayout {
    /// Number of rows in the grid
    pub rows: usize,
    /// Number of columns in the grid
    pub cols: usize,
}

impl GridLayout {
    /// Create a new grid layout
    #[must_use]
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

/// 2D dimensions (typically width × height)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    /// Width
    pub width: f32,
    /// Height
    pub height: f32,
}

impl Dimensions {
    /// Create new dimensions
    #[must_use]
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

/// 2D scaling factors
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scale {
    /// Horizontal scaling factor
    pub x: f32,
    /// Vertical scaling factor
    pub y: f32,
}

impl Scale {
    /// Create new scale factors
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Layout parameters for imposing pages: grid, dimensions, and scaling
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImpositionLayout {
    /// Grid layout (rows × columns)
    pub grid: GridLayout,
    /// Dimensions of each page slot on the output page
    pub slot: Dimensions,
    /// Output page dimensions
    pub output: Dimensions,
    /// Source page dimensions
    pub source: Dimensions,
    /// Scaling factors
    pub scale: Scale,
}

impl ImpositionLayout {
    /// Create new imposition layout parameters
    #[must_use]
    pub fn new(
        grid: GridLayout,
        slot: Dimensions,
        output: Dimensions,
        source: Dimensions,
        scale: Scale,
    ) -> Self {
        Self {
            grid,
            slot,
            output,
            source,
            scale,
        }
    }
}

/// Pages to impose on a single sheet
#[derive(Debug, Clone, Copy)]
pub struct SheetPages<'a> {
    /// All pages from the source document with their object IDs
    pub all_pages: &'a [(u32, lopdf::ObjectId)],
    /// Page numbers to place on this sheet (1-indexed, 0 = blank page)
    pub page_nums: &'a [usize],
}

impl<'a> SheetPages<'a> {
    /// Create a new sheet pages reference
    #[must_use]
    pub fn new(all_pages: &'a [(u32, lopdf::ObjectId)], page_nums: &'a [usize]) -> Self {
        Self {
            all_pages,
            page_nums,
        }
    }
}
