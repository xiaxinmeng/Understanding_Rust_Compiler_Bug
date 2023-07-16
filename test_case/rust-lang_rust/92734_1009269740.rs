plain
[RUSTC-TIMING] structopt test:false 0.054
error: unused imports: `PathBuf`, `Path`
  --> src/tools/rustfmt/src/ignore_path.rs:35:21
   |
35 |     use std::path::{Path, PathBuf};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused imports: `Config`, `FileName`
  --> src/tools/rustfmt/src/ignore_path.rs:37:25
   |
   |
37 |     use crate::config::{Config, FileName};

error: unused import: `crate::ignore_path::IgnorePathSet`
  --> src/tools/rustfmt/src/ignore_path.rs:38:9
   |
