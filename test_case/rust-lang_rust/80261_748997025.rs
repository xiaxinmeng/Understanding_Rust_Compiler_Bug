plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.085 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii......ii...ii..........iiii..........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.44s

 finished in 2.528 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
running 5 tests
..F.F
failures:

---- util::comments::tests::test_block_doc_comment_2 stdout ----
thread 'util::comments::tests::test_block_doc_comment_2' panicked at 'assertion failed: `(left == right)`
  left: `" * Test\n *  Test"`,
 right: `" Test\n  Test"`', compiler/rustc_ast/src/util/comments/tests.rs:18:9


---- util::comments::tests::test_block_doc_comment_1 stdout ----
thread 'util::comments::tests::test_block_doc_comment_1' panicked at 'assertion failed: `(left == right)`
  left: `" * Test \n **  Test\n *   Test"`,
 right: `" Test \n*  Test\n   Test"`', compiler/rustc_ast/src/util/comments/tests.rs:9:9

failures:
    util::comments::tests::test_block_doc_comment_1
    util::comments::tests::test_block_doc_comment_2
    util::comments::tests::test_block_doc_comment_2

test result: FAILED. 3 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '-p rustc_ast --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_ast" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:24:47
