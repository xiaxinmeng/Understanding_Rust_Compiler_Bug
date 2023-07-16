plain
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 351 tests
ii.............i....i..ii.................iii........ii.i.......i.................ii.... 88/351
.............i............i..i.................i.F.iii........i..i......i.i........i..ii 176/351
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
i.i.ii.i......i.......iii.......i...i.....................iiiiiiii.i...................
failures:


---- [codegen] src/test/codegen/intrinsics/mask.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/mask/mask.ll" "/checkout/src/test/codegen/intrinsics/mask.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/intrinsics/mask.rs:9:17: error: CHECK-SAME: expected string not found in input
 // CHECK-SAME: @llvm.ptrmask.{{p0|p0i8}}.[[WORD]](ptr %ptr, [[WORD]] %mask)
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/mask/mask.ll:10:16: note: scanning from here
 %1 = tail call i8* @llvm.ptrmask.p0i8.i64(i8* %0, i64 %mask)
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/mask/mask.ll:10:16: note: with "WORD" equal to "i64"
 %1 = tail call i8* @llvm.ptrmask.p0i8.i64(i8* %0, i64 %mask)
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/mask/mask.ll:10:16: note: with "WORD" equal to "i64"
 %1 = tail call i8* @llvm.ptrmask.p0i8.i64(i8* %0, i64 %mask)
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/mask/mask.ll:10:21: note: possible intended match here
 %1 = tail call i8* @llvm.ptrmask.p0i8.i64(i8* %0, i64 %mask)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/mask/mask.ll
Check file: /checkout/src/test/codegen/intrinsics/mask.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
          .
          .
          .
          .
          5:  
          6: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind readnone uwtable willreturn 
          7: define i16* @mask_ptr(i16* readnone %ptr, i64 %mask) unnamed_addr #0 { 
          8: start: 
          9:  %0 = bitcast i16* %ptr to i8* 
         10:  %1 = tail call i8* @llvm.ptrmask.p0i8.i64(i8* %0, i64 %mask) 
same:9'0                    X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
same:9'1                                                                    with "WORD" equal to "i64"
same:9'2                                                                    with "WORD" equal to "i64"
same:9'3                         ?                                          possible intended match
         11:  %2 = bitcast i8* %1 to i16* 
same:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         12:  ret i16* %2 
same:9'0     ~~~~~~~~~~~~~
         13: } 
same:9'0     ~~
         14:  
same:9'0     ~
         15: ; Function Attrs: mustprogress nofree nosync nounwind readnone speculatable willreturn 
same:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          .
          .
>>>>>>
------------------------------------------
