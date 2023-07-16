plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....iiiiii..................
failures:

---- [codegen] codegen/autovectorize-f32x4.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/autovectorize-f32x4/autovectorize-f32x4.ll" "/checkout/src/test/codegen/autovectorize-f32x4.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/autovectorize-f32x4.rs:20:17: error: CHECK-LABEL: expected string not found in input
// CHECK-LABEL: @auto_vectorize_loop
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/autovectorize-f32x4/autovectorize-f32x4.ll:10:35: note: scanning from here
define void @auto_vectorize_direct([4 x float]* noalias nocapture noundef sret([4 x float]) dereferenceable(16) %0, [4 x float]* noalias nocapture noundef readonly dereferenceable(16) %a, [4 x float]* noalias nocapture noundef readonly dereferenceable(16) %b) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/autovectorize-f32x4/autovectorize-f32x4.ll:22:23: note: possible intended match here
; autovectorize_f32x4::auto_vectorize_loop


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/autovectorize-f32x4/autovectorize-f32x4.ll
Check file: /checkout/src/test/codegen/autovectorize-f32x4.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
            5: 
            6: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
            7: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }
            8: 
            9: ; Function Attrs: nofree norecurse nounwind nonlazybind uwtable willreturn
           10: define void @auto_vectorize_direct([4 x float]* noalias nocapture noundef sret([4 x float]) dereferenceable(16) %0, [4 x float]* noalias nocapture noundef readonly dereferenceable(16) %a, [4 x float]* noalias nocapture noundef readonly dereferenceable(16) %b) unnamed_addr #0 {
label:20'0                                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           11: start:
label:20'0     ~~~~~~
           12:  %1 = bitcast [4 x float]* %a to <4 x float>*
label:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  %2 = load <4 x float>, <4 x float>* %1, align 4
label:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  %3 = bitcast [4 x float]* %b to <4 x float>*
label:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15:  %4 = load <4 x float>, <4 x float>* %3, align 4
label:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  %5 = fadd <4 x float> %2, %4
label:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17:  %6 = bitcast [4 x float]* %0 to <4 x float>*
label:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18:  store <4 x float> %5, <4 x float>* %6, align 4
label:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19:  ret void
label:20'0     ~~~~~~~~~
           20: }
label:20'0     ~
           21: 
label:20'0     ~
           22: ; autovectorize_f32x4::auto_vectorize_loop
label:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
label:20'1                           ?                    possible intended match
           23: ; Function Attrs: nonlazybind uwtable
label:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24: define void @_ZN19autovectorize_f32x419auto_vectorize_loop17h2f3e09e31fd9487aE([4 x float]* noalias nocapture noundef sret([4 x float]) dereferenceable(16) %c, [4 x float]* noalias nocapture noundef readonly dereferenceable(16) %a, [4 x float]* noalias nocapture noundef readonly dereferenceable(16) %b) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
label:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25: start:
label:20'0     ~~~~~~
           26:  %0 = bitcast [4 x float]* %a to <4 x float>*
label:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27:  %1 = load <4 x float>, <4 x float>* %0, align 4
label:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------



failures:
    [codegen] codegen/autovectorize-f32x4.rs
test result: FAILED. 275 passed; 1 failed; 52 ignored; 0 measured; 0 filtered out; finished in 3.59s

Build completed unsuccessfully in 0:12:27
