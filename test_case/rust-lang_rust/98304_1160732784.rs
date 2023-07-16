plain
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 339 tests
ii....i........i....i..i..................iii.......ii.i.......i.................ii..... 88/339
............i..............i................i....iii.......i..i........i.......i.F.....i 176/339
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....i.......iii..........i.......................iiiiiiii..................
failures:


---- [codegen] src/test/codegen/issue-96274.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-96274/issue-96274.ll" "/checkout/src/test/codegen/issue-96274.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/issue-96274.rs:8:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: memset
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-96274/issue-96274.ll:11:23: note: found here
 tail call void @llvm.memset.p0i8.i64(i8* nonnull align 1 dereferenceable(3000) %1, i8 undef, i64 3000, i1 false)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-96274/issue-96274.ll
Check file: /checkout/src/test/codegen/issue-96274.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
       .
       .
       .
       .
       6: ; issue_96274::maybe_uninit
       7: ; Function Attrs: nofree nounwind nonlazybind uwtable willreturn writeonly
       8: define void @_ZN11issue_9627412maybe_uninit17he7b7007fb747cb0fE([3000 x i8]* noalias nocapture noundef sret([3000 x i8]) dereferenceable(3000) %0) unnamed_addr #0 {
       9: start:
      10:  %1 = getelementptr inbounds [3000 x i8], [3000 x i8]* %0, i64 0, i64 0
      11:  tail call void @llvm.memset.p0i8.i64(i8* nonnull align 1 dereferenceable(3000) %1, i8 undef, i64 3000, i1 false)
not:8                           !~~~~~                                                                                      error: no match expected
      12:  ret void
      13: }
      14: 
      15: ; Function Attrs: argmemonly nofree nosync nounwind willreturn writeonly
      16: declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1
       .
       .
>>>>>>
------------------------------------------
