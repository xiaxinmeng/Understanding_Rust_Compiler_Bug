plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking core v0.0.0 (/checkout/library/core)
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
    Checking compiler_builtins v0.1.66
 Documenting alloc v0.0.0 (/checkout/library/alloc)
error: `char` is both a module and a builtin type
   |
   |
90 | /// You can append a [`char`] to a `String` with the [`push`] method, and
   |                        ^^^^ ambiguous link
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
help: to link to the module, prefix with `mod@`
   |
90 | /// You can append a [`mod@char`] to a `String` with the [`push`] method, and
   |                        ++++
help: to link to the builtin type, prefix with `prim@`
   |
90 | /// You can append a [`prim@char`] to a `String` with the [`push`] method, and


error: `char` is both a module and a builtin type
    |
    |
857 |     /// Panics if the starting point or end point do not lie on a [`char`]
    |                                                                     ^^^^ ambiguous link
    |
help: to link to the module, prefix with `mod@`
    |
857 |     /// Panics if the starting point or end point do not lie on a [`mod@char`]
    |                                                                     ++++
help: to link to the builtin type, prefix with `prim@`
    |
857 |     /// Panics if the starting point or end point do not lie on a [`prim@char`]


error: `char` is both a module and a builtin type
     |
     |
1128 |     /// Appends the given [`char`] to the end of this `String`.
     |                             ^^^^ ambiguous link
     |
help: to link to the module, prefix with `mod@`
     |
1128 |     /// Appends the given [`mod@char`] to the end of this `String`.
     |                             ++++
help: to link to the builtin type, prefix with `prim@`
     |
1128 |     /// Appends the given [`prim@char`] to the end of this `String`.


error: `char` is both a module and a builtin type
     |
     |
1195 |     /// Panics if `new_len` does not lie on a [`char`] boundary.
     |                                                 ^^^^ ambiguous link
     |
help: to link to the module, prefix with `mod@`
     |
1195 |     /// Panics if `new_len` does not lie on a [`mod@char`] boundary.
     |                                                 ++++
help: to link to the builtin type, prefix with `prim@`
     |
1195 |     /// Panics if `new_len` does not lie on a [`prim@char`] boundary.


error: `char` is both a module and a builtin type
     |
     |
1245 |     /// Removes a [`char`] from this `String` at a byte position and returns it.
     |                     ^^^^ ambiguous link
     |
help: to link to the module, prefix with `mod@`
     |
1245 |     /// Removes a [`mod@char`] from this `String` at a byte position and returns it.
     |                     ++++
help: to link to the builtin type, prefix with `prim@`
     |
1245 |     /// Removes a [`prim@char`] from this `String` at a byte position and returns it.


error: `char` is both a module and a builtin type
     |
     |
1253 |     /// or if it does not lie on a [`char`] boundary.
     |                                      ^^^^ ambiguous link
     |
help: to link to the module, prefix with `mod@`
     |
1253 |     /// or if it does not lie on a [`mod@char`] boundary.
     |                                      ++++
help: to link to the builtin type, prefix with `prim@`
     |
1253 |     /// or if it does not lie on a [`prim@char`] boundary.


error: `char` is both a module and a builtin type
     |
     |
1437 |     /// lie on a [`char`] boundary.
     |                    ^^^^ ambiguous link
     |
help: to link to the module, prefix with `mod@`
     |
1437 |     /// lie on a [`mod@char`] boundary.
     |                    ++++
help: to link to the builtin type, prefix with `prim@`
     |
1437 |     /// lie on a [`prim@char`] boundary.


error: `char` is both a module and a builtin type
     |
     |
1486 |     /// lie on a [`char`] boundary.
     |                    ^^^^ ambiguous link
     |
help: to link to the module, prefix with `mod@`
     |
1486 |     /// lie on a [`mod@char`] boundary.
     |                    ++++
help: to link to the builtin type, prefix with `prim@`
     |
1486 |     /// lie on a [`prim@char`] boundary.


error: `char` is both a module and a builtin type
     |
     |
1541 |     /// Returns the length of this `String`, in bytes, not [`char`]s or
     |                                                              ^^^^ ambiguous link
     |
help: to link to the module, prefix with `mod@`
     |
1541 |     /// Returns the length of this `String`, in bytes, not [`mod@char`]s or
     |                                                              ++++
help: to link to the builtin type, prefix with `prim@`
     |
1541 |     /// Returns the length of this `String`, in bytes, not [`prim@char`]s or


error: `char` is both a module and a builtin type
     |
     |
1649 |     /// Panics if the starting point or end point do not lie on a [`char`]
     |                                                                     ^^^^ ambiguous link
     |
help: to link to the module, prefix with `mod@`
     |
1649 |     /// Panics if the starting point or end point do not lie on a [`mod@char`]
     |                                                                     ++++
help: to link to the builtin type, prefix with `prim@`
     |
1649 |     /// Panics if the starting point or end point do not lie on a [`prim@char`]


error: `char` is both a module and a builtin type
     |
     |
1699 |     /// Panics if the starting point or end point do not lie on a [`char`]
     |                                                                     ^^^^ ambiguous link
     |
help: to link to the module, prefix with `mod@`
     |
1699 |     /// Panics if the starting point or end point do not lie on a [`mod@char`]
     |                                                                     ++++
help: to link to the builtin type, prefix with `prim@`
     |
1699 |     /// Panics if the starting point or end point do not lie on a [`prim@char`]

error: could not document `alloc`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name alloc library/alloc/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.60.0 --index-page /checkout/src/doc/index.md -C metadata=de98b983d6499a71 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-72c436c2eb83779d.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta --cfg=bootstrap -Zsymbol-mangling-version=legacy -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.60.0-nightly
  (5d2a2a119
  2022-01-30)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.60.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:00:20
