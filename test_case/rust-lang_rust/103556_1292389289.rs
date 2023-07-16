plain

running 377 tests
i.....i..............i....i..ii.................iii........ii.i........i................ 88/377
.ii.................i............i..i.................i...iii........i..i......iii.ii... 176/377
.....i.iii....i...................i...i...i.....ii..i.ii...F.............ii............. 264/377
iiii.i...................
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [codegen] src/test/codegen/option-nonzero-eq.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/option-nonzero-eq/option-nonzero-eq.ll" "/checkout/src/test/codegen/option-nonzero-eq.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/option-nonzero-eq.rs:20:17: error: CHECK-NEXT: expected string not found in input
 // CHECK-NEXT: %2 = icmp eq i64 %0, %1
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/option-nonzero-eq/option-nonzero-eq.ll:17:7: note: scanning from here
      ^
      ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/option-nonzero-eq/option-nonzero-eq.ll:18:2: note: possible intended match here
 %2 = icmp eq i8* %0, %1


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/option-nonzero-eq/option-nonzero-eq.ll
Check file: /checkout/src/test/codegen/option-nonzero-eq.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
          12:  ret i1 %2 
          13: } 
          14:  
          15: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind uwtable willreturn 
          16: define noundef zeroext i1 @non_null_eq(i8* %0, i8* %1) unnamed_addr #0 { 
          17: start: 
next:20'0           X error: no match found
          18:  %2 = icmp eq i8* %0, %1 
next:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
next:20'1      ?                        possible intended match
          19:  ret i1 %2 
next:20'0     ~~~~~~~~~~~
          20: } 
next:20'0     ~~
          21:  
next:20'0     ~
          22: attributes #0 = { mustprogress nofree nosync nounwind nonlazybind uwtable willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
next:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          23:  
next:20'0     ~
           .
           .
>>>>>>
------------------------------------------
