plain
.................................................................................................... 9400/11740
.................................................................................................... 9500/11740
................................................................................i.......i........... 9600/11740
.................................................................................................... 9700/11740
..........................iiiiiii..iiiiii.i......................................................... 9800/11740
.................................................................................................... 10000/11740
.................................................................................................... 10100/11740
.................................................................................................... 10200/11740
.................................................................................................... 10300/11740
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.04s

 finished in 0.105 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i....i......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.430 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
doc tests for: /checkout/src/doc/unstable-book/src/language-features/generators.md
doc tests for: /checkout/src/doc/unstable-book/src/language-features/half-open-range-patterns.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/src/doc/unstable-book/src/language-features/half-open-range-patterns.md" "--test-args" ""

stdout ----

running 1 test
running 1 test
test /checkout/src/doc/unstable-book/src/language-features/half-open-range-patterns.md - The_tracking_issue_for_this_feature_is__ (line 18) ... FAILED

failures:

---- /checkout/src/doc/unstable-book/src/language-features/half-open-range-patterns.md - The_tracking_issue_for_this_feature_is__ (line 18) stdout ----
error[E0004]: non-exhaustive patterns: `10_i32..=i32::MAX` not covered
 --> /checkout/src/doc/unstable-book/src/language-features/half-open-range-patterns.md:22:11
6 |     match x {
6 |     match x {
  |           ^ pattern `10_i32..=i32::MAX` not covered
  = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
  = note: the matched value is of type `i32`

error: aborting due to previous error
