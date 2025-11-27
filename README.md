# imposer

A small, focused Rust library and CLI for arranging PDF pages into booklet layouts (n-up) with common binding options.

## Features

- **Flexible n-up Imposition**: Support for power-of-two pages-per-sheet (2-up, 4-up, 8-up, 16-up, 32-up, ...)
- **Multiple Binding Types**: Saddle stitch and perfect binding with full mathematical accuracy
- **Automatic Padding**: Intelligently pads to maintain proper sheet divisibility
- **Correct Page Ordering**: Proper duplex booklet page ordering for accurate printing

## Binding Types Explained

### Saddle Stitch (Nested Binding)

Saddle stitch is the traditional booklet binding where pages are nested together. Each sheet wraps the previous one, creating a nested structure.

**Structure:**

```
Sheet 1 (outer):  Front: [Last, First]        Back: [Second, Last-1]
Sheet 2 (inner):  Front: [Last-2, Third]      Back: [Fourth, Last-3]
Sheet 3 (inner):  Front: [Last-4, Fifth]      Back: [Sixth, Last-5]
                  ... (continues inward)
```

**Example: 8 pages, 2-up saddle stitch:**

```
Sheet 1:  Front: [8, 1]    Back: [2, 7]
Sheet 2:  Front: [6, 3]    Back: [4, 5]
         ↓ Nest sheets (Sheet 2 inside Sheet 1)
Result: Pages fold correctly from outside to inside
```

### Perfect Binding (Signatures Stacking)

Perfect binding groups pages into **signatures**, where each signature internally uses saddle-stitch ordering, but signatures are then **stacked sequentially** (not nested). This is how paperback books are bound.

**Structure:**

```
Signature 1: Pages 1-8    (internally: saddle-stitch ordered)
Signature 2: Pages 9-16   (internally: saddle-stitch ordered)
Signature 3: Pages 17-24  (internally: saddle-stitch ordered)
             ↓ Stack signatures (Sig 2 behind Sig 1, etc.)
```

## Key Concept: Sheets Per Signature

The `sheets_per_signature` parameter controls how many physical sheets go into each signature:

- **1 sheet per signature**: Each signature contains a single nested sheet. The entire booklet is one large stacking of signatures. For 8 pages with 2-up and 1 sheet/sig, you get 2 signatures of 4 pages each.

- **2+ sheets per signature**: Multiple sheets are nested within each signature before stacking. For 8 pages with 2-up and 2 sheets/sig, you get 1 signature where both sheets are nested (identical to pure saddle stitch).

## Important: Physical Sheets vs. Printing Sheets

The `sheets_per_signature` parameter refers to **physical sheets after cutting**. Different n-up values produce different numbers of physical sheets from one printing sheet:

- **2-up**: 1 printing sheet → 1 physical sheet
- **4-up**: 1 printing sheet → 2 physical sheets (cut horizontally)
- **8-up**: 1 printing sheet → 4 physical sheets (cut horizontally and vertically)

This means `--perfect-binding --sheets-per-signature 2 --2up` produces the **same page grouping** as `--perfect-binding --sheets-per-signature 4 --4up`, just with different grid layouts.

## Example: 16 pages perfect binding comparison

```
Saddle Stitch (single signature):
  Sheet 1: [16, 1, 14, 3] / [2, 15, 4, 13]
  Sheet 2: [12, 5, 10, 7] / [6, 11, 8, 9]
  (all nested together)

Perfect Binding with 1 sheet/sig:
  Signature 1 (pages 1-8):
    Sheet 1: [8, 1, 6, 3] / [2, 7, 4, 5]
  Signature 2 (pages 9-16):
    Sheet 1: [16, 9, 14, 11] / [10, 15, 12, 13]
  (signatures stacked)

Perfect Binding with 2 sheets/sig:
  Signature 1 (pages 1-16):
    Sheet 1: [16, 1, 14, 3] / [2, 15, 4, 13]
    Sheet 2: [12, 5, 10, 7] / [6, 11, 8, 9]
  (identical to pure saddle stitch)
```

## Algorithm Implementation

The imposition algorithms correctly:

- Pad odd-page inputs to the next multiple of `pages_per_sheet`
- Calculate optimal grid layout for page arrangement
- Apply proper page reversal for duplex printing (varies by n-up value)
- Handle arbitrary power-of-2 n-up values
- Preserve blank pages in mathematically correct positions for proper folding
- Verified with 109-page test across multiple n-up values (all pages present, no duplicates)

## Documentation

Find the crate and detailed documentation online:

- [crates.io: imposer](https://crates.io/crates/imposer)
- [docs.rs: imposer](https://docs.rs/imposer)

## Notes and Capabilities

- Assumes reasonably uniform page sizes across input PDF
- Pads output with blank pages for proper sheet divisibility
- Blank pages appear in mathematically correct positions (not consolidated)

## Command-line options reference

### Required options

- **`-i, --input <PATH>`**: Input PDF file path. Must be a valid PDF.
- **`-o, --output <PATH>`**: Output PDF file path. The generated booklet will be written here.

### Layout and binding options

- **`-n, --pages-per-sheet <N>`** (default: 2): Number of pages to arrange per sheet side. Must be a power of 2 for saddle stitch (2, 4, 8, 16, 32, 64, ...). No restriction for perfect binding.

- **`-p, --page-size <SIZE>`** (default: a4): Output page size. Valid options: `a4`, `a3`, `a5`, `letter`, `legal`, `tabloid`.

- **`--perfect-binding`**: Use perfect binding instead of saddle stitch. Perfect binding stacks signatures sequentially (like a paperback book) rather than nesting them. Cannot be used with saddle-stitch-specific settings.

### Perfect binding signature control

**Note**: These options only apply when `--perfect-binding` is used.

- **`--sheets-per-signature <N>`** (default: 1): Number of physical sheets to nest within each signature. A value of 1 creates simple stacking; higher values create sewn signatures with nested sheets inside.

- **`--num-signatures <N>`** (optional): If specified, pages are evenly distributed across this many signatures. This takes precedence over `--sheets-per-signature`. Useful when you want a specific number of signature sections regardless of content.

**Important note on `--sheets-per-signature` and `--num-signatures`**: These are mutually exclusive concepts:

- Use `--sheets-per-signature` when you want to control the physical structure of each signature (how many sheets are nested per signature).
- Use `--num-signatures` when you want to control the number of signature sections the document is divided into. If both are specified, `--num-signatures` takes precedence.

### Scaling and appearance options

- **`--no-scale-to-fit`**: Do not scale source pages to fit the output page size. By default, pages are scaled to fill the available space while respecting margins.

- **`--no-preserve-aspect-ratio`**: Allow pages to be stretched or compressed to fill the output page. By default, aspect ratio is preserved, which may result in letterboxing or pillarboxing.

- **`--draw-guides`**: Draw fold and cut guide lines on the output. Useful for debugging and visualizing the page layout before printing.

- **`--number-pages`**: Replace page content with page numbers. Useful for visualizing the page order without needing to view the actual PDF content.

### Default behavior

If you run `imposer -i input.pdf -o output.pdf` with no other options:

- Uses **saddle stitch** binding
- Creates a **2-up** layout (2 pages per sheet)
- Output size is **A4**
- Pages are **scaled to fit** the output size while **preserving aspect ratio**
- No guides or page numbers are drawn

## Building

```bash
cargo build --release
```

## Try it

```bash
cargo run --bin imposer -- -i input.pdf -o booklet.pdf
```

## Installation

Using the binary (when Cargo bin is in your PATH)

If you prefer to run `imposer` directly from your shell without `cargo run`, install the binary into your Cargo bin directory (usually `~/.cargo/bin`) and make sure that directory is on your PATH.

- Install locally from the repository (installs into `~/.cargo/bin`):

```bash
cargo install --path .
```

- Or install the published crate from crates.io:

```bash
cargo install imposer
```

- Ensure `~/.cargo/bin` is on your PATH (add to your shell profile if necessary):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
# Add the line above to ~/.bashrc, ~/.zshrc, or your shell profile to make it persistent
```

## Using imposer from the command line

Once `imposer` is installed and in your PATH, you can run it directly:

```bash
imposer -i input.pdf -o output.pdf
```

### Common usage examples

**Saddle stitch 2-up (default):**

```bash
imposer -i input.pdf -o booklet.pdf
```

**Saddle stitch 4-up (4 pages per sheet):**

```bash
imposer -i input.pdf -o booklet.pdf -n 4
```

**Perfect binding with 2 sheets per signature, 2-up layout:**

```bash
imposer -i input.pdf -o book.pdf --perfect-binding --sheets-per-signature 2
```

**Perfect binding 4-up (larger pages per sheet):**

```bash
imposer -i input.pdf -o book.pdf --perfect-binding -n 4
```

All options and defaults are available via the help command:

```bash
imposer --help
```

## Python bindings

`imposer` also provides Python bindings via PyO3, allowing you to use the library directly in Python code. This is useful for:
- Integrating PDF imposition into Python applications
- Batch processing PDFs programmatically
- Embedding booklet generation in web services
- Automating workflows in tools like Scribus

### Installation

Install the Python package from PyPI:

```bash
pip install imposer
```

Or install locally from the repository:

```bash
pip install .
```

### Basic usage

```python
import imposer

# Create a default 2-up A4 saddle-stitch booklet
config = imposer.BookletConfig()
imposer.generate_booklet_from_file("input.pdf", "output.pdf", config)
```

### Configuration options

```python
import imposer

# Configure a custom booklet
binding = imposer.BindingType.perfect_bound(
    sheets_per_signature=8,  # 8 sheets per signature
    num_signatures=3         # 3 total signatures
)

config = (
    imposer.BookletConfig()
    .with_page_size(imposer.PageSize.a4())
    .with_pages_per_sheet(4)
    .with_binding_type(binding)
    .with_draw_guides(True)       # Draw cut/fold lines
    .with_number_pages(True)      # Add page numbers
    .with_scale_to_fit(True)      # Scale pages to fit
    .with_preserve_aspect_ratio(True)
)

imposer.generate_booklet_from_file("input.pdf", "output.pdf", config)
```

### In-memory processing

Process PDFs without intermediate files:

```python
import imposer

# Read input PDF
with open("input.pdf", "rb") as f:
    input_bytes = f.read()

# Generate booklet configuration
config = imposer.BookletConfig()

# Process in memory
output_bytes = imposer.generate_booklet(input_bytes, config)

# Write output
with open("output.pdf", "wb") as f:
    f.write(output_bytes)
```

### Available page sizes

```python
imposer.PageSize.a4()      # A4 (210 × 297 mm)
imposer.PageSize.a3()      # A3 (297 × 420 mm)
imposer.PageSize.a5()      # A5 (148 × 210 mm)
imposer.PageSize.letter()  # Letter (8.5 × 11 in)
imposer.PageSize.legal()   # Legal (8.5 × 14 in)
imposer.PageSize.tabloid() # Tabloid (11 × 17 in)
```

### Binding types

```python
# Saddle-stitch (default): pages nested, stapled in the middle
binding = imposer.BindingType.saddle_stitch()

# Perfect binding: signatures stacked, glued at spine
binding = imposer.BindingType.perfect_bound(
    sheets_per_signature=4,    # Pages per signature (default: 1)
    num_signatures=None        # Override with specific signature count
)
```

### Examples

See `examples/python_example.py` for detailed examples including:
- Basic booklet creation
- Different page sizes
- Various n-up layouts
- Perfect binding with signatures
- Guides and page numbers
- In-memory processing
- Scribus integration

Run the example:

```bash
python examples/python_example.py input.pdf
```
