plain
    Checking core v0.0.0 (/checkout/library/core)
   Compiling compiler_builtins v0.1.55
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
 Documenting alloc v0.0.0 (/checkout/library/alloc)
error: `core::cmp::Ord` is both a trait and a macro
    |
    |
254 | /// [`Ord`]: core::cmp::Ord
    |              ^^^^^^^^^^^^^^ ambiguous link
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
help: to link to the trait, prefix with `trait@`
    |
254 | /// [`Ord`]: trait@core::cmp::Ord
    |              ++++++
help: to link to the macro, add an exclamation mark
    |
254 | /// [`Ord`]: core::cmp::Ord!


error: `core::cmp::Ord` is both a trait and a macro
   |
   |
30 | /// [`Ord`]: core::cmp::Ord
   |              ^^^^^^^^^^^^^^ ambiguous link
   |
help: to link to the trait, prefix with `trait@`
   |
30 | /// [`Ord`]: trait@core::cmp::Ord
   |              ++++++
help: to link to the macro, add an exclamation mark
   |
30 | /// [`Ord`]: core::cmp::Ord!


error: `core::cmp::Ord` is both a trait and a macro
     |
     |
2802 | /// Implements ordering of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison).
     |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ambiguous link
     |
help: to link to the trait, prefix with `trait@`
     |
2802 | /// Implements ordering of vectors, [lexicographically](trait@core::cmp::Ord#lexicographical-comparison).
     |                                                         ++++++
help: to link to the macro, add an exclamation mark
     |
2802 | /// Implements ordering of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison!).


error: `core::cmp::Ord` is both a trait and a macro
     |
     |
2790 | /// Implements comparison of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison).
     |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ambiguous link
     |
help: to link to the trait, prefix with `trait@`
     |
2790 | /// Implements comparison of vectors, [lexicographically](trait@core::cmp::Ord#lexicographical-comparison).
     |                                                           ++++++
help: to link to the macro, add an exclamation mark
     |
2790 | /// Implements comparison of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison!).

error: could not document `alloc`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name alloc library/alloc/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.59.0 --index-page /checkout/src/doc/index.md -C metadata=2b278315e6dda61e -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-2c9c844da8a1e155.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-24cf655a97be028a.rmeta -Zsymbol-mangling-version=legacy -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.59.0-nightly
  (d684a5624
  2021-12-11)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.59.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:25:44
