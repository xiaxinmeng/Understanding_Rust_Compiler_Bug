plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
  |
9 | use rustc_data_structures::sync::Lrc;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: unused imports: `FilePathMapping`, `SourceMap`
   |
   |
22 | use rustc_span::source_map::{FilePathMapping, SourceMap};

error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:01:42
