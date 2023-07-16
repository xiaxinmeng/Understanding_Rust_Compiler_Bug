plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.064 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii..........i....i...i.......ii..iii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.03s

 finished in 2.097 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
   Compiling difference v2.0.0
   Compiling once_cell v1.4.1
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0277]: `format::Buffer` doesn't implement `std::fmt::Display`
   --> src/librustdoc/html/highlight/tests.rs:24:58
    |
24  |           format!("{}<pre><code>{}</code></pre>\n", STYLE, out)
    |           -------------------------------------------------^^^-
    |           |                                                |
    |           |                                                `format::Buffer` cannot be formatted with the default formatter
    |           in this macro invocation (#1)
   ::: /checkout/library/alloc/src/macros.rs:110:1
    |
110 | / macro_rules! format {
111 | |     ($($arg:tt)*) => {{
111 | |     ($($arg:tt)*) => {{
112 | |         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
    | |                                       ---------------------------------------- in this macro invocation (#2)
114 | |     }}
115 | | }
115 | | }
    | |_- in this expansion of `format!` (#1)
   ::: /checkout/library/core/src/macros/mod.rs:749:5
    |
749 | /     macro_rules! format_args {
749 | /     macro_rules! format_args {
750 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
751 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
752 | |     }
    | |_____- in this expansion of `$crate::__export::format_args!` (#2)
    |
    = help: the trait `std::fmt::Display` is not implemented for `format::Buffer`
    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    = note: required by `std::fmt::Display::fmt`

error[E0599]: no method named `into_inner` found for struct `std::string::String` in the current scope
  --> src/librustdoc/html/highlight/tests.rs:26:58
   |
26 |     expect_file!["fixtures/sample.html"].assert_eq(&html.into_inner());
   |                                                          ^^^^^^^^^^ method not found in `std::string::String`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustdoc`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:25:02
