plain
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0432]: unresolved import `crate::clean::collapse_doc_fragments`
 --> src/librustdoc/clean/types/tests.rs:3:5
  |
3 | use crate::clean::collapse_doc_fragments;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `collapse_doc_fragments` in `clean`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustdoc` (lib test) due to previous error
Build completed unsuccessfully in 0:21:50
