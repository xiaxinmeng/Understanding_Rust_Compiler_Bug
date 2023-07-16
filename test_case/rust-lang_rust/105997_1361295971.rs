plain
i.....i...............i...i..ii.................iii........ii.i........i................ 88/392
...ii.................i............i..i..............i....i....iii.......i..i.....i.iii. 176/392
iii.........i.iii...i..i...................i...ii....i....ii..i.ii....i...............ii 264/392
........................i.i.ii.i.i............i..i....i....i..iii.......i...ii.......... 352/392
...........iiiiiiiiFi............F......
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [codegen] src/test/codegen/unwind-abis/c-unwind-abi-panic-abort.rs stdout ----


error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-abis/c-unwind-abi-panic-abort/c-unwind-abi-panic-abort.ll" "/checkout/src/test/codegen/unwind-abis/c-unwind-abi-panic-abort.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/unwind-abis/c-unwind-abi-panic-abort.rs:12:12: error: CHECK: expected string not found in input
 // CHECK: call void @_ZN4core9panicking15panic_no_unwind
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-abis/c-unwind-abi-panic-abort/c-unwind-abi-panic-abort.ll:10:57: note: scanning from here
define void @rust_item_that_can_unwind() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                                                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-abis/c-unwind-abi-panic-abort/c-unwind-abi-panic-abort.ll:19:7: note: possible intended match here
 tail call void @_ZN4core9panicking19panic_cannot_unwind17hbb91975a01a62040E() #3

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-abis/c-unwind-abi-panic-abort/c-unwind-abi-panic-abort.ll
Check file: /checkout/src/test/codegen/unwind-abis/c-unwind-abi-panic-abort.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
            5:  
            6: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [2 x i64] } 
            7: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
            8:  
            9: ; Function Attrs: nounwind nonlazybind 
           10: define void @rust_item_that_can_unwind() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
check:12'0                                                             X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           11: start: 
check:12'0     ~~~~~~~
           12:  invoke void @may_unwind() 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  to label %bb1 unwind label %cleanup 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  
check:12'0     ~
           15: cleanup: ; preds = %start 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  %0 = landingpad { i8*, i32 } 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17:  cleanup 
check:12'0     ~~~~~~~~~
           18: ; call core::panicking::panic_cannot_unwind 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19:  tail call void @_ZN4core9panicking19panic_cannot_unwind17hbb91975a01a62040E() #3 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:12'1           ?                                                                            possible intended match
           20:  unreachable 
check:12'0     ~~~~~~~~~~~~~
           21:  
check:12'0     ~
           22: bb1: ; preds = %start 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~
           23:  ret void 
check:12'0     ~~~~~~~~~~
           24: } 
check:12'0     ~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/unwind-and-panic-abort.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-and-panic-abort/unwind-and-panic-abort.ll" "/checkout/src/test/codegen/unwind-and-panic-abort.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/unwind-and-panic-abort.rs:12:11: error: CHECK: expected string not found in input
// CHECK: call void @_ZN4core9panicking15panic_no_unwind
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-and-panic-abort/unwind-and-panic-abort.ll:10:17: note: scanning from here
define void @foo() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-and-panic-abort/unwind-and-panic-abort.ll:19:7: note: possible intended match here
 tail call void @_ZN4core9panicking19panic_cannot_unwind17hbb91975a01a62040E() #3

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-and-panic-abort/unwind-and-panic-abort.ll
Check file: /checkout/src/test/codegen/unwind-and-panic-abort.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
            5:  
            6: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [2 x i64] } 
            7: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
            8:  
            9: ; Function Attrs: nounwind nonlazybind 
           10: define void @foo() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
check:12'0                     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           11: start: 
check:12'0     ~~~~~~~
           12:  invoke void @bar() 
check:12'0     ~~~~~~~~~~~~~~~~~~~~
           13:  to label %bb1 unwind label %cleanup 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  
check:12'0     ~
           15: cleanup: ; preds = %start 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  %0 = landingpad { i8*, i32 } 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17:  cleanup 
check:12'0     ~~~~~~~~~
           18: ; call core::panicking::panic_cannot_unwind 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19:  tail call void @_ZN4core9panicking19panic_cannot_unwind17hbb91975a01a62040E() #3 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:12'1           ?                                                                            possible intended match
           20:  unreachable 
check:12'0     ~~~~~~~~~~~~~
           21:  
check:12'0     ~
           22: bb1: ; preds = %start 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~
           23:  ret void 
check:12'0     ~~~~~~~~~~
           24: } 
check:12'0     ~~
            .
            .
>>>>>>
------------------------------------------
