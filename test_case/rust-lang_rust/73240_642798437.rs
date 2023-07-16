
failures:

---- [codegen] codegen/sanitizer-no-sanitize-inlining.rs#ASAN stdout ----


------------------------------------------
stderr:
------------------------------------------
/Users/runner/runners/2.170.1/work/1/s/src/test/codegen/sanitizer-no-sanitize-inlining.rs:16:10: error: ASAN: expected string not found in input
// ASAN:***@random_inline
         ^
/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/test/codegen/sanitizer-no-sanitize-inlining.ASAN/sanitizer-no-sanitize-inlining.ll:9:18: note: scanning from here
define void @test(i32* nocapture align 4 dereferenceable(4) %n) unnamed_addr #0 {
                 ^
/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/test/codegen/sanitizer-no-sanitize-inlining.ASAN/sanitizer-no-sanitize-inlining.ll:11:2: note: possible intended match here
 tail call void @random_inline(i32* nonnull align 4 dereferenceable(4) %n)
 ^

------------------------------------------



failures:
    [codegen] codegen/sanitizer-no-sanitize-inlining.rs#ASAN

test result: FAILED. 165 passed; 1 failed; 29 ignored; 0 measured; 0 filtered out
