plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0433]: failed to resolve: use of undeclared crate or module `kw`
  --> src/librustdoc/passes/unindent_comments.rs:90:28
   |
90 |         if fragment.doc == kw::Empty {
   |                            ^^ use of undeclared crate or module `kw`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:20
