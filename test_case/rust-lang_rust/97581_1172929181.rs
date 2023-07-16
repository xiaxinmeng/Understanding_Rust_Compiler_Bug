plain
failures:

---- [codegen] src/test/codegen/vec-calloc-2.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-calloc-2/vec-calloc-2.ll" "/checkout/src/test/codegen/vec-calloc-2.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/vec-calloc-2.rs:21:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: reserve
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-calloc-2/vec-calloc-2.ll:44:71: note: found here
 br i1 %8, label %bb20.i.i.i, label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h0e71061e4f872fabE.exit.i.i.i"


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-calloc-2/vec-calloc-2.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
       39:  br label %bb19.i.i.i 
       40:  
       41: bb19.i.i.i: ; preds = %bb1.i.i.i.i.i, %bb2.i.i.i.i.i 
       42:  %result.sroa.0.0.i.i.i = phi i8* [ %6, %bb2.i.i.i.i.i ], [ %7, %bb1.i.i.i.i.i ] 
       43:  %8 = icmp eq i8* %result.sroa.0.0.i.i.i, null 
       44:  br i1 %8, label %bb20.i.i.i, label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h0e71061e4f872fabE.exit.i.i.i" 
not:21                                                                           !~~~~~~                                  error: no match expected
       45:  
       46: bb20.i.i.i: ; preds = %bb19.i.i.i 
       47: ; call alloc::alloc::handle_alloc_error 
       48:  call void @_ZN5alloc5alloc18handle_alloc_error17h999a9a6ed7016a50E(i64 %3, i64 noundef %spec.select.i.i.i.i) #8, !noalias !8 
       49:  unreachable 
        .
        .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/vec-calloc.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-calloc/vec-calloc.ll" "/checkout/src/test/codegen/vec-calloc.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/vec-calloc.rs:71:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: reserve
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-calloc/vec-calloc.ll:165:71: note: found here
 br i1 %8, label %bb20.i.i.i, label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h7def89784916f02eE.exit.i.i.i"
                                                                      ^~~~~~~
/checkout/src/test/codegen/vec-calloc.rs:139:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: reserve
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-calloc/vec-calloc.ll:523:71: note: found here
 br i1 %8, label %bb20.i.i.i, label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h1fd1a61ddd7d6766E.exit.i.i.i"

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-calloc/vec-calloc.ll
Check file: /checkout/src/test/codegen/vec-calloc.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
         .
         .
         .
         .
       160:  br label %bb19.i.i.i 
       161:  
       162: bb19.i.i.i: ; preds = %bb1.i.i.i.i.i, %bb2.i.i.i.i.i 
       163:  %result.sroa.0.0.i.i.i = phi i8* [ %6, %bb2.i.i.i.i.i ], [ %7, %bb1.i.i.i.i.i ] 
       164:  %8 = icmp eq i8* %result.sroa.0.0.i.i.i, null 
       165:  br i1 %8, label %bb20.i.i.i, label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h7def89784916f02eE.exit.i.i.i" 
not:71                                                                            !~~~~~~                                  error: no match expected
       166:  
       167: bb20.i.i.i: ; preds = %bb19.i.i.i 
       168: ; call alloc::alloc::handle_alloc_error 
       169:  tail call void @_ZN5alloc5alloc18handle_alloc_error17h999a9a6ed7016a50E(i64 %3, i64 noundef %spec.select.i.i.i.i) #9, !noalias !29 
       170:  unreachable 
         .
         .
         .
       518:  br label %bb19.i.i.i 
       519:  
       520: bb19.i.i.i: ; preds = %bb1.i.i.i.i.i, %bb2.i.i.i.i.i 
       521:  %result.sroa.0.0.i.i.i = phi i8* [ %6, %bb2.i.i.i.i.i ], [ %7, %bb1.i.i.i.i.i ] 
       522:  %8 = icmp eq i8* %result.sroa.0.0.i.i.i, null 
       523:  br i1 %8, label %bb20.i.i.i, label %"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h1fd1a61ddd7d6766E.exit.i.i.i" 
not:139                                                                           !~~~~~~                                  error: no match expected
       524:  
       525: bb20.i.i.i: ; preds = %bb19.i.i.i 
       526: ; call alloc::alloc::handle_alloc_error 
       527:  tail call void @_ZN5alloc5alloc18handle_alloc_error17h999a9a6ed7016a50E(i64 %3, i64 noundef %spec.select.i.i.i.i) #9, !noalias !70 
       528:  unreachable 
         .
         .
>>>>>>
------------------------------------------
