plain

failures:
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [codegen] src/test/codegen/unchecked_shifts.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked_shifts/unchecked_shifts.ll" "/checkout/src/test/codegen/unchecked_shifts.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/unchecked_shifts.rs:22:16: error: CHECK-DAG: expected string not found in input
 // CHECK-DAG: %[[INRANGE:.+]] = icmp ult i32 %b, 65536
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked_shifts/unchecked_shifts.ll:20:43: note: scanning from here
define i16 @unchecked_shl_unsigned_smaller(i16 %a, i32 %b) unnamed_addr #1 personality ptr @rust_eh_personality {
                                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked_shifts/unchecked_shifts.ll:22:5: note: possible intended match here
 %_2.i.i = icmp ugt i32 %b, 65535
    ^
/checkout/src/test/codegen/unchecked_shifts.rs:52:16: error: CHECK-DAG: expected string not found in input
 // CHECK-DAG: %[[INRANGE:.+]] = icmp ult i32 %b, 32768
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked_shifts/unchecked_shifts.ll:61:41: note: scanning from here
define i16 @unchecked_shr_signed_smaller(i16 %a, i32 %b) unnamed_addr #1 personality ptr @rust_eh_personality {
                                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked_shifts/unchecked_shifts.ll:63:5: note: possible intended match here
 %_2.i.i = icmp ugt i32 %b, 32767


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked_shifts/unchecked_shifts.ll
Check file: /checkout/src/test/codegen/unchecked_shifts.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
          .
          .
          .
          .
         15:  %0 = shl i32 %a, %b 
         16:  ret i32 %0 
         17: } 
         18:  
         19: ; Function Attrs: nonlazybind uwtable 
         20: define i16 @unchecked_shl_unsigned_smaller(i16 %a, i32 %b) unnamed_addr #1 personality ptr @rust_eh_personality { 
dag:22'0                                               X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
         21: start: 
dag:22'0     ~~~~~~~
         22:  %_2.i.i = icmp ugt i32 %b, 65535 
dag:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
dag:22'1         ?                              possible intended match
         23:  br i1 %_2.i.i, label %bb2.i, label %"_ZN4core6option15Option$LT$T$GT$16unwrap_unchecked17h5fc80b933cf12eb7E.exit" 
dag:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         24:  
dag:22'0     ~
         25: bb2.i: ; preds = %start 
dag:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~
         26: ; call core::panicking::panic 
dag:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         27:  tail call void @_ZN4core9panicking5panic17h4a2384573d0594c7E(ptr noalias noundef nonnull readonly align 1 @alloc49, i64 32, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) @alloc53) #3 
dag:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          .
          .
          .
         56:  %0 = ashr i32 %a, %b 
         57:  ret i32 %0 
         58: } 
         59:  
         60: ; Function Attrs: nonlazybind uwtable 
         61: define i16 @unchecked_shr_signed_smaller(i16 %a, i32 %b) unnamed_addr #1 personality ptr @rust_eh_personality { 
dag:52'0                                             X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
         62: start: 
dag:52'0     ~~~~~~~
         63:  %_2.i.i = icmp ugt i32 %b, 32767 
dag:52'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
dag:52'1         ?                              possible intended match
         64:  br i1 %_2.i.i, label %bb2.i, label %"_ZN4core6option15Option$LT$T$GT$16unwrap_unchecked17hd6ddc5ba5706fe00E.exit" 
dag:52'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         65:  
dag:52'0     ~
         66: bb2.i: ; preds = %start 
dag:52'0     ~~~~~~~~~~~~~~~~~~~~~~~~
         67: ; call core::panicking::panic 
dag:52'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         68:  tail call void @_ZN4core9panicking5panic17h4a2384573d0594c7E(ptr noalias noundef nonnull readonly align 1 @alloc49, i64 32, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) @alloc59) #3 
dag:52'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          .
          .
>>>>>>
------------------------------------------
