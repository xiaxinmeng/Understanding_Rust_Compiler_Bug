plain
    Checking clippy_utils v0.1.54 (/checkout/src/tools/clippy/clippy_utils)
    Checking url v2.2.2
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.54 (/checkout/src/tools/clippy/clippy_lints)
error: variable `edition` is assigned to, but never used
    |
487 |     let mut edition = None;
    |             ^^^^^^^
    |
    |
    = note: `-D unused-variables` implied by `-D warnings`
    = note: consider using `_edition` instead

error: value assigned to `edition` is never read
    |
    |
500 | ...                   edition = stripped.parse::<Edition>().ok();
    |
    |
    = note: `-D unused-assignments` implied by `-D warnings`
    = help: maybe it is overwritten before being read?
error: aborting due to 2 previous errors

error: could not compile `clippy_lints`

