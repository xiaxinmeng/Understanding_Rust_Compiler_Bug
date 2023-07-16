plain
.................................................................................................... 9400/11740
.................................................................................................... 9500/11740
................................................................................i......i............ 9600/11740
.................................................................................................... 9700/11740
..........................iiiiiii...iiiiiii......................................................... 9800/11740
.................................................................................................... 10000/11740
.................................................................................................... 10100/11740
.................................................................................................... 10200/11740
.................................................................................................... 10300/11740
---
 finished in 0.413 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.103 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.46s

Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
 finished in 2.531 seconds
---
doc tests for: /checkout/src/doc/unstable-book/src/language-features/doc-notable-trait.md
doc tests for: /checkout/src/doc/unstable-book/src/language-features/exclusive-range-pattern.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/src/doc/unstable-book/src/language-features/exclusive-range-pattern.md" "--test-args" ""

stdout ----

running 1 test
