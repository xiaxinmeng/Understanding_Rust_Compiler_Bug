plain
    Checking getopts v0.2.21
error[E0432]: unresolved import `rustc_errors::emitter::MAX_SUGGESTION_HIGHLIGHT_LINES`
  --> src/tools/clippy/clippy_utils/src/diagnostics.rs:11:20
   |
11 | use rustc_errors::{emitter::MAX_SUGGESTION_HIGHLIGHT_LINES, Applicability, Diagnostic, MultiSpan};
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `MAX_SUGGESTION_HIGHLIGHT_LINES` in `emitter`
   Compiling tokio v1.8.4
   Compiling libz-sys v1.1.3
    Checking pulldown-cmark v0.9.1
    Checking aho-corasick v0.7.18
