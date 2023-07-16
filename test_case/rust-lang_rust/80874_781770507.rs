plain
.................................................................................................... 9200/11468
.................................................................................................... 9300/11468
.................................................................................................... 9400/11468
..........................i......i.................................................................. 9500/11468
..................................................................iiiiiii.iiiiii.i.................. 9600/11468
.................................................................................................... 9800/11468
.................................................................................................... 9900/11468
.................................................................................................... 10000/11468
.................................................................................................... 10100/11468
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.081 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.40s

 finished in 2.483 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
doc tests for: /checkout/src/doc/rustdoc/src/how-to-write-documentation.md
doc tests for: /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Winvalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/src/doc/rustdoc/src/linking-to-items-by-name.md" "--test-args" ""

stdout ----

running 7 tests
running 7 tests
test /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Warnings__re_exports__and_scoping (line 112) ... FAILED
test /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Namespaces_and_Disambiguators (line 80) ... ok
test /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Valid_links (line 67) ... ok
test /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Warnings__re_exports__and_scoping (line 126) ... ok
test /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Namespaces_and_Disambiguators (line 96) ... ok
test /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Valid_links (line 48) ... ok

failures:
failures:

---- /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Warnings__re_exports__and_scoping (line 112) stdout ----
error[E0432]: unresolved import `inner`
 --> /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md:118:9
  |
8 | pub use inner::S; // the link to `f` will still resolve correctly
  |         ^^^^^ maybe a missing crate `inner`?
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    /checkout/src/doc/rustdoc/src/linking-to-items-by-name.md - Linking_to_items_by_name::Warnings__re_exports__and_scoping (line 112)
test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s


stderr ----
