plain
.................................................................................................... 9000/11196
.................................................................................................... 9100/11196
......................................................................................i......i...... 9200/11196
.................................................................................................... 9300/11196
.........................iiiiii..iiiiii.i........................................................... 9400/11196
.................................................................................................... 9600/11196
.................................................................................................... 9700/11196
.................................................................................................... 9800/11196
.................................................................................................... 9900/11196
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.075 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i.....ii.ii.....ii....ii...........iiii........i.....i....i......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.38s

 finished in 2.464 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
 --> src/librustdoc/passes/unindent_comments/tests.rs:9:14
  |
9 |         doc: s.to_string(),
  |              ^^^^^^^^^^^^^ expected struct `rustc_span::Symbol`, found struct `std::string::String`

error[E0063]: missing fields `indent`, `need_backline` in initializer of `clean::types::DocFragment`
 --> src/librustdoc/passes/unindent_comments/tests.rs:5:10
  |
5 |     vec![DocFragment {
  |          ^^^^^^^^^^^ missing `indent`, `need_backline`
error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:44:35
   |
40 | / macro_rules! assert_eq {
40 | / macro_rules! assert_eq {
41 | |     ($left:expr, $right:expr $(,)?) => ({
42 | |         match (&$left, &$right) {
43 | |             (left_val, right_val) => {
44 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected struct `rustc_span::Symbol`, found `&str`
69 | |     });
70 | | }
70 | | }
   | |_- in this expansion of `assert_eq!`
  ::: src/librustdoc/passes/unindent_comments/tests.rs:18:5
   |
   |
18 |       assert_eq!(s[0].doc, expected);

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0063, E0308.
Some errors have detailed explanations: E0063, E0308.
For more information about an error, try `rustc --explain E0063`.
error: could not compile `rustdoc`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:29:22
