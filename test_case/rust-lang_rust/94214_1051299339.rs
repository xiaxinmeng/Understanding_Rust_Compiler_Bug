plain
Suite(test::src/test/codegen) not skipped for "bootstrap::test::Codegen" -- not in [src/tools/tidy]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 328 tests
ii....i...F....i...i..i................iii.......iii.......i.................ii.................i... 100/328
i.ii.............iiii........................i.ii.i......i.......iii..........i...................ii 300/328
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...iiiiii...................
failures:
failures:

---- [codegen] codegen/adjustments.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll" "/checkout/src/test/codegen/adjustments.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/adjustments.rs:16:11: error: CHECK: expected string not found in input
// CHECK: %0 = insertvalue { {{[0 x i8]\*|ptr}}, [[USIZE]] } undef, {{[0 x i8]\*|ptr}} %x.0, 0
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll:13:50: note: scanning from here
define { [0 x i8]*, i64 } @no_op_slice_adjustment([0 x i8]* noalias noundef nonnull readonly align 1 %x.0, i64 %x.1) unnamed_addr #0 {
                                                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll:13:50: note: with "USIZE" equal to "i64"
define { [0 x i8]*, i64 } @no_op_slice_adjustment([0 x i8]* noalias noundef nonnull readonly align 1 %x.0, i64 %x.1) unnamed_addr #0 {
                                                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll:15:2: note: possible intended match here
 %0 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %x.0, 0

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll
Check file: /checkout/src/test/codegen/adjustments.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
            8: start:
            9:  ret void
           10: }
           11: 
           12: ; Function Attrs: nonlazybind uwtable
           13: define { [0 x i8]*, i64 } @no_op_slice_adjustment([0 x i8]* noalias noundef nonnull readonly align 1 %x.0, i64 %x.1) unnamed_addr #0 {
check:16'0                                                      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:16'1                                                                                                                                            with "USIZE" equal to "i64"
           14: start:
check:16'0     ~~~~~~
           15:  %0 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %x.0, 0
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'2      ?                                                            possible intended match
           16:  %1 = insertvalue { [0 x i8]*, i64 } %0, i64 %x.1, 1
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17:  ret { [0 x i8]*, i64 } %1
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           18: }
check:16'0     ~
           19: 
check:16'0     ~
           20: ; Function Attrs: nonlazybind uwtable
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
