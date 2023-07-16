plain
.................................................................................................... 9000/11185
.................................................................................................... 9100/11185
............................................................................i......i................ 9200/11185
.................................................................................................... 9300/11185
...............iiiiii..iiiiii.i..................................................................... 9400/11185
.................................................................................................... 9600/11185
.................................................................................................... 9700/11185
.......................test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
............................................................................. 9800/11185
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.073 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i..i...ii...........iiii........i.....i...i.......ii.i.ii.....iiii...... 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.49s

 finished in 2.569 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.............................................................................iiii................... 1100/1145
.............................................
failures:

---- src/io/mod.rs - io::read_to_string (line 957) stdout ----
error[E0658]: use of unstable library feature 'io_read_to_string'
  |
  |
5 |     let stdin = io::read_to_string(io::stdin());
  |
  = note: see issue #80218 <https://github.com/rust-lang/rust/issues/80218> for more information
  = note: see issue #80218 <https://github.com/rust-lang/rust/issues/80218> for more information
  = help: add `#![feature(io_read_to_string)]` to the crate attributes to enable
error[E0308]: mismatched types
 --> src/io/mod.rs:960:36
  |
  |
5 |     let stdin = io::read_to_string(io::stdin());
  |                                    |
  |                                    |
  |                                    expected `&mut _`, found struct `Stdin`
  |                                    help: consider mutably borrowing here: `&mut io::stdin()`
  = note: expected mutable reference `&mut _`
                        found struct `Stdin`


error[E0277]: `std::result::Result<String, std::io::Error>` doesn't implement `std::fmt::Display`
  |
7 |     println!("{}", stdin);
7 |     println!("{}", stdin);
  |                    ^^^^^ `std::result::Result<String, std::io::Error>` cannot be formatted with the default formatter
  |
  = help: the trait `std::fmt::Display` is not implemented for `std::result::Result<String, std::io::Error>`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
  = note: required by `std::fmt::Display::fmt`
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308, E0658.
For more information about an error, try `rustc --explain E0277`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:26:52
