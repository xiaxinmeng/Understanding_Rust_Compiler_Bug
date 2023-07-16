plain
 finished in 0.436 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.072 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 9.717 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i..i...ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.403 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Set({"library/term"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
 finished in 67.172 seconds
Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling term v0.0.0 (/checkout/library/term)
error: panic message is not a string literal
  --> library/term/src/terminfo/parm/tests.rs:80:30
   |
80 |         assert!(res.is_ok(), res.unwrap_err());
   |
   |
   = note: `-D non-fmt-panic` implied by `-D warnings`
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
80 |         assert!(res.is_ok(), "{}", res.unwrap_err());

error: panic message is not a string literal
error: panic message is not a string literal
  --> library/term/src/terminfo/parm/tests.rs:84:30
   |
84 |         assert!(res.is_ok(), res.unwrap_err());
   |
   = note: this is no longer accepted in Rust 2021
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
84 |         assert!(res.is_ok(), "{}", res.unwrap_err());

error: panic message is not a string literal
error: panic message is not a string literal
  --> library/term/src/terminfo/parm/tests.rs:88:30
   |
88 |         assert!(res.is_ok(), res.unwrap_err());
   |
   = note: this is no longer accepted in Rust 2021
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
88 |         assert!(res.is_ok(), "{}", res.unwrap_err());

error: panic message is not a string literal
error: panic message is not a string literal
  --> library/term/src/terminfo/parm/tests.rs:98:26
   |
98 |     assert!(res.is_ok(), res.unwrap_err());
   |
   = note: this is no longer accepted in Rust 2021
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
98 |     assert!(res.is_ok(), "{}", res.unwrap_err());

error: panic message is not a string literal
error: panic message is not a string literal
   --> library/term/src/terminfo/parm/tests.rs:101:26
    |
101 |     assert!(res.is_ok(), res.unwrap_err());
    |
    = note: this is no longer accepted in Rust 2021
    = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
    |
101 |     assert!(res.is_ok(), "{}", res.unwrap_err());

error: panic message is not a string literal
error: panic message is not a string literal
   --> library/term/src/terminfo/parm/tests.rs:104:26
    |
104 |     assert!(res.is_ok(), res.unwrap_err());
    |
    = note: this is no longer accepted in Rust 2021
    = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
    |
104 |     assert!(res.is_ok(), "{}", res.unwrap_err());

error: aborting due to 6 previous errors

error: could not compile `term`
error: could not compile `term`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "term" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:13
