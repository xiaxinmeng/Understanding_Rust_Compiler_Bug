plain
.................................................................................................... 9000/11189
.................................................................................................... 9100/11189
................................................................................i......i............ 9200/11189
.................................................................................................... 9300/11189
...................iiiiii..iiiiii.i................................................................. 9400/11189
.................................................................................................... 9600/11189
.................................................................................................... 9700/11189
..........................................................................................test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
.......... 9800/11189
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.071 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i...i..i..ii....ii....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.39s

 finished in 2.458 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
   Compiling once_cell v1.4.1
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
  --> src/librustdoc/html/highlight/tests.rs:21:9
   |
21 |         write_code(&mut out, src);
   |         |
   |         expected 3 arguments
   |
note: function defined here
note: function defined here
  --> src/librustdoc/html/highlight.rs:50:4
   |
50 | fn write_code(out: &mut String, src: &str, edition: Edition) {

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
  --> src/librustdoc/html/highlight/tests.rs:33:5
33 |     write_code(&mut html, src);
   |     ^^^^^^^^^^ ---------  --- supplied 2 arguments
   |     |
   |     expected 3 arguments
   |     expected 3 arguments
   |
note: function defined here
  --> src/librustdoc/html/highlight.rs:50:4
   |
50 | fn write_code(out: &mut String, src: &str, edition: Edition) {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0061`.
For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustdoc`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:29:20
