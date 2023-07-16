plain
failures:

---- [codegen] codegen/sparc-struct-abi.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sparc-struct-abi/sparc-struct-abi.ll" "/checkout/src/test/codegen/sparc-struct-abi.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/sparc-struct-abi.rs:58:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: ret { i32, double, i32 } { i32 8060928, double 3.140000e+00, i32 poison }
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sparc-struct-abi/sparc-struct-abi.ll:27:2: note: scanning from here
 ret { i32, double, i32 } { i32 8060928, double 3.140000e+00, i32 undef }

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sparc-struct-abi/sparc-struct-abi.ll
Check file: /checkout/src/test/codegen/sparc-struct-abi.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
         .
         .
         .
         .
        22: }
        23: 
        24: ; Function Attrs: norecurse nounwind nonlazybind readnone uwtable willreturn
        25: define { i32, double, i32 } @structshortdouble() unnamed_addr #0 {
        26: start:
        27:  ret { i32, double, i32 } { i32 8060928, double 3.140000e+00, i32 undef }
next:58      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
        28: }
next:58     ~
next:58     ~
next:58     ~
        30: ; Function Attrs: norecurse nounwind nonlazybind readnone uwtable willreturn
next:58     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        31: define void @structshortdouble_input({ i32, double, i32 } %0) unnamed_addr #0 {
next:58     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
next:58     ~~~~~~
         .
         .
         .
