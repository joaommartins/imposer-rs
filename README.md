# imposer

A small, focused Rust library and CLI for arranging PDF pages into booklet layouts (n-up) with common binding options.

## Docs

Find the crate and documentation online:

- [crates.io: imposer](https://crates.io/crates/imposer)
- [docs.rs: imposer](https://docs.rs/imposer)

## Features

- **Flexible n-up Saddle Stitch Imposition**: Support for power-of-two pages-per-sheet (2-up, 4-up, 8-up, 16-up, 32-up, ...). Choose the n-up value that fits your printer and folding strategy.
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

## Building

```bash
cargo build --release

Try it

```bash
cargo run --bin imposer -- -i input.pdf -o booklet.pdf
```

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
