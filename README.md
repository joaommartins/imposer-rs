# imposer

A Rust library and CLI for imposing single-sheet PDFs into booklet layouts with configurable n-up binding.

## Features

- **Library API**: Call `imposer::generate_booklet` with PDF bytes and configuration to get back imposed PDF bytes
- **CLI**: `imposer --input in.pdf --output booklet.pdf [--pages-per-sheet N] [--binding-type TYPE]`
- **Flexible n-up Saddle Stitch Imposition**: Support for any power of 2 pages per sheet (2-up, 4-up, 8-up, 16-up, 32-up, 64-up, 128-up, 256-up, 512-up, 1024-up, ...)
- **Multiple Binding Types**:
  - **Saddle Stitch** (nested): Traditional booklet binding with pages arranged in nested signatures
  - **Perfect Binding** (sequential): Book binding where pages are stacked sequentially (WIP)
- **Automatic Padding**: Rounds to nearest multiple of pages-per-sheet for proper imposition
- **Correct Page Ordering**: Implements proper duplex booklet page ordering for accurate printing

## Algorithm

The saddle stitch imposition algorithm correctly:

- Pads odd-page inputs to the next multiple of `pages_per_sheet`
- Calculates optimal grid layout for page arrangement
- Applies proper page reversal for duplex printing
- Handles arbitrary power-of-2 n-up values
- Verified with 64-up imposition on 109-page test (all pages present, no duplicates)

## Notes and Capabilities

- Assumes reasonably uniform page sizes across input PDF
- Pads output with blank pages for proper sheet divisibility
- Modularized imposition module with separate algorithms for each binding type

Building

    cargo build --release

Running

    cargo run --bin imposer -- -i input.pdf -o booklet.pdf
