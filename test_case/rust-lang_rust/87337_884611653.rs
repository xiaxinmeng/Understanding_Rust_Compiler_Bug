plain
Suite("src/test/rustdoc-ui") not skipped for "bootstrap::test::RustdocUi" -- not in ["src/tools/tidy"]
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 124 tests
..F..............F..FF.F...FFF....F.....F....F...........FFF.FFF.F.FFF.F.....FFF.........F.......... 100/124
thread 'main' panicked at 'I/O failure during tests: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }', src/tools/compiletest/src/main.rs:424:13
....F...FF..FF.FF.F...F.
failures:

---- [ui] rustdoc-ui/check-attr-test.rs stdout ----


error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/check-attr-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-attr-test" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-attr-test/auxiliary"
------------------------------------------

running 0 tests


test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


------------------------------------------
stderr:
------------------------------------------
error: unknown attribute `compile-fail`. Did you mean `compile_fail`?
  |
  |
5 | / /// foo
6 | | ///
7 | | /// 