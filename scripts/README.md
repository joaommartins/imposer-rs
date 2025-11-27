# Scripts

This directory contains utility scripts for the imposer project.

## check_bindings_sync.py

Verifies that Python bindings (in `src/python.rs`) are in sync with the Rust library's public API.

**Usage:**
```bash
python3 scripts/check_bindings_sync.py
```

**What it checks:**
- All public Rust `PageSize` variants have corresponding Python methods
- All public Rust `BindingType` variants are exposed in Python
- All `BookletConfig` builder methods (`with_*`) are available in Python
- Core public functions like `generate_booklet` are exposed to Python

**When to run:**
- Automatically runs in CI during pull requests
- Run manually before committing changes to `src/config.rs` or `src/python.rs`
- Run after adding new public API items to ensure bindings are updated

**Exit codes:**
- 0: All bindings are in sync
- 1: Bindings are out of sync (see error output)
