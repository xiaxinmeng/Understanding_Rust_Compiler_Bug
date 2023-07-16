plain

Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [codegen] src/test/codegen/enum-match.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/enum-match/enum-match.ll" "/checkout/src/test/codegen/enum-match.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/enum-match.rs:37:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: %1 = tail call i8 @llvm.usub.sat.i8(i8 %0, i8 1)
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/enum-match/enum-match.ll:17:7: note: scanning from here
      ^
      ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/enum-match/enum-match.ll:18:2: note: possible intended match here
 %1 = call i8 @llvm.usub.sat.i8(i8 %0, i8 1)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/enum-match/enum-match.ll
Check file: /checkout/src/test/codegen/enum-match.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
          12:  ret i8 %.0 
          13: } 
          14:  
          15: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind readnone uwtable willreturn 
          16: define i8 @match1(i8 noundef %0) unnamed_addr #1 { 
          17: start: 
next:37'0           X error: no match found
          18:  %1 = call i8 @llvm.usub.sat.i8(i8 %0, i8 1) 
next:37'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
next:37'1      ?                                            possible intended match
          19:  switch i8 %1, label %bb2 [ 
next:37'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          20:  i8 0, label %bb3 
next:37'0     ~~~~~~~~~~~~~~~~~~
          21:  i8 1, label %bb5 
next:37'0     ~~~~~~~~~~~~~~~~~~
          22:  i8 2, label %bb1 
next:37'0     ~~~~~~~~~~~~~~~~~~
          23:  ] 
next:37'0     ~~~
           .
           .
>>>>>>
------------------------------------------
