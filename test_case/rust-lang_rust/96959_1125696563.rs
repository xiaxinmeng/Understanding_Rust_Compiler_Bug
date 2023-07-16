plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=i686-unknown-linux-musl

---- [codegen] src/test/codegen/unwind-abis/c-unwind-abi-panic-abort.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-abis/c-unwind-abi-panic-abort/c-unwind-abi-panic-abort.ll" "/checkout/src/test/codegen/unwind-abis/c-unwind-abi-panic-abort.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/unwind-abis/c-unwind-abi-panic-abort.rs:17:12: error: CHECK: expected string not found in input
 // CHECK: @may_unwind() unnamed_addr #1
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-abis/c-unwind-abi-panic-abort/c-unwind-abi-panic-abort.ll:16:53: note: scanning from here
 tail call void @_ZN4core9panicking15panic_no_unwind17hc74ada0432bbf7c6E() #4
                                                    ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-abis/c-unwind-abi-panic-abort/c-unwind-abi-panic-abort.ll:27:14: note: possible intended match here
declare void @may_unwind() unnamed_addr #2

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-abis/c-unwind-abi-panic-abort/c-unwind-abi-panic-abort.ll
Check file: /checkout/src/test/codegen/unwind-abis/c-unwind-abi-panic-abort.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           11:  
           12: cleanup: ; preds = %start 
           13:  %0 = landingpad { i8*, i32 } 
           14:  cleanup 
           15: ; call core::panicking::panic_no_unwind 
           16:  tail call void @_ZN4core9panicking15panic_no_unwind17hc74ada0432bbf7c6E() #4 
check:17'0                                                         X~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           17:  unreachable 
check:17'0     ~~~~~~~~~~~~~
           18:  
check:17'0     ~
           19: bb1: ; preds = %start 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~
           20:  ret void 
check:17'0     ~~~~~~~~~~
           21: } 
check:17'0     ~~
           22:  
check:17'0     ~
           23: ; Function Attrs: nonlazybind 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24: declare i32 @rust_eh_personality(...) unnamed_addr #1 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25:  
check:17'0     ~
           26: ; Function Attrs: nonlazybind 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27: declare void @may_unwind() unnamed_addr #2 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:17'1                  ?                              possible intended match
           28:  
check:17'0     ~
           29: ; core::panicking::panic_no_unwind 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           30: ; Function Attrs: cold noinline noreturn nounwind nonlazybind 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31: declare void @_ZN4core9panicking15panic_no_unwind17hc74ada0432bbf7c6E() unnamed_addr #3 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           32:  
check:17'0     ~
            .
            .
>>>>>>
------------------------------------------
