plain
.................................................................................................... 9300/11515
.................................................................................................... 9400/11515
..................................................................i......i.......................... 9500/11515
.................................................................................................... 9600/11515
.....iiiiiii..iiiiii.i.............................................................................. 9700/11515
.................................................................................................... 9900/11515
.................................................................................................... 10000/11515
.................................................................................................... 10100/11515
.................................................................................................... 10200/11515
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.068 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.27s

 finished in 2.347 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
doc tests for: /checkout/src/doc/rustdoc/src/passes.md
doc tests for: /checkout/src/doc/rustdoc/src/the-doc-attribute.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Winvalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/src/doc/rustdoc/src/the-doc-attribute.md" "--test-args" ""

stdout ----

running 15 tests
---
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::_ (line 170) ... ok

failures:

---- /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_crate_level::By_default__ (line 118) stdout ----
error: unknown `doc` attribute `html_no_source`
 --> /checkout/src/doc/rustdoc/src/the-doc-attribute.md:118:8
  |
2 | #![doc(html_no_source)]

error: aborting due to previous error

Couldn't compile the test.
