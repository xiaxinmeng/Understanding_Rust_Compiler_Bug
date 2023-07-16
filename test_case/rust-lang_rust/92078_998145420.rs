plain
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0432]: unresolved import `crate::clean::collapse_doc_fragments`
 --> src/librustdoc/passes/unindent_comments/tests.rs:3:5
  |
3 | use crate::clean::collapse_doc_fragments;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `collapse_doc_fragments` in `clean`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustdoc` due to previous error



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:30:56
