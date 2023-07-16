plain
failures:

---- [codegen] codegen/vec-shrink-panik.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik/vec-shrink-panik.ll" "/checkout/src/test/codegen/vec-shrink-panik.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/vec-shrink-panik.rs:19:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik/vec-shrink-panik.ll:199:14: note: found here
; call core::panicking::panic_no_unwind
             ^~~~~
/checkout/src/test/codegen/vec-shrink-panik.rs:26:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik/vec-shrink-panik.ll:336:14: note: found here
; call core::panicking::panic_no_unwind

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik/vec-shrink-panik.ll
Check file: /checkout/src/test/codegen/vec-shrink-panik.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
      194:  unreachable 
      195:  
      196: abort.i: ; preds = %bb12.i 
      197:  %17 = landingpad { i8*, i32 } 
      198:  cleanup 
      199: ; call core::panicking::panic_no_unwind 
not:19                  !~~~~                       error: no match expected
      200:  tail call void @_ZN4core9panicking15panic_no_unwind17h4e371630455be247E() #13, !noalias !47 
      201:  unreachable 
      202:  
      203: bb9.i: ; preds = %bb12.i 
      204:  resume { i8*, i32 } %18 
        .
        .
      331:  unreachable 
      332:  
      332:  
      333: abort.i.i.i: ; preds = %bb12.i.i.i 
      334:  %26 = landingpad { i8*, i32 } 
      335:  cleanup 
      336: ; call core::panicking::panic_no_unwind 
not:26                  !~~~~                       error: no match expected
      337:  tail call void @_ZN4core9panicking15panic_no_unwind17h4e371630455be247E() #13, !noalias !129 
      338:  unreachable 
      339:  
      340: bb9.i.i.i: ; preds = %bb12.i.i.i 
      341:  resume { i8*, i32 } %27 
        .
        .
>>>>>>
------------------------------------------
