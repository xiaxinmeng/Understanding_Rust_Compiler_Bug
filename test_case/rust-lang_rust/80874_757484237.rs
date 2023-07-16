plain
.................................................................................................... 9000/11246
.................................................................................................... 9100/11246
.................................................................................................... 9200/11246
..........................................i.......i................................................. 9300/11246
.................................................................................iiiiii..iiiiii.i... 9400/11246
.................................................................................................... 9600/11246
.................................................................................................... 9700/11246
.................................................................................................... 9800/11246
.................................................................................................... 9900/11246
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.064 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i....ii..i.i....ii..........iiii.........i.....i...i.......ii.i..ii....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 1.89s

 finished in 1.961 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
doc tests for: /checkout/src/doc/rustdoc/src/how-to-write-documentation.md
doc tests for: /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Winvalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/src/doc/rustdoc/src/linking-to-items-by-name.md" "--test-args" ""

stdout ----

running 6 tests
running 6 tests
test /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Warnings__re_exports__and_scoping (line 123) ... FAILED
test /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Namespaces_and_Disambiguators (line 88) ... ok
test /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Valid_links (line 73) ... ok
test /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Namespaces_and_Disambiguators (line 104) ... ok
test /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Valid_links (line 54) ... ok

failures:


---- /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Warnings__re_exports__and_scoping (line 123) stdout ----
error: expected one of `::`, `;`, or `as`, found keyword `pub`
 --> /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md:127:1
4 | pub use std::process::Command
4 | pub use std::process::Command
  |                              - expected one of `::`, `;`, or `as`
5 | 
6 | pub fn foo() {}

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.

failures:
    /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Warnings__re_exports__and_scoping (line 123)
test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s


stderr ----
