plain

Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [codegen] src/test/codegen/sanitizer-cfi-emit-type-checks.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-checks/sanitizer-cfi-emit-type-checks.ll" "/checkout/src/test/codegen/sanitizer-cfi-emit-type-checks.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/sanitizer-cfi-emit-type-checks.rs:11:17: error: CHECK-NEXT: is not on the line after the previous match
 // CHECK-NEXT: [[TT:%.+]] = call i1 @llvm.type.test({{i8\*|ptr}} {{%f|%0}}, metadata !"{{[[:print:]]+}}")
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-checks/sanitizer-cfi-emit-type-checks.ll:11:2: note: 'next' match was here
 %1 = call i1 @llvm.type.test(i8* %0, metadata !"_ZTSFu3i32S_E")
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-checks/sanitizer-cfi-emit-type-checks.ll:9:7: note: previous match ended here
      ^
      ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-checks/sanitizer-cfi-emit-type-checks.ll:10:1: note: non-matching line after previous match is here
 %0 = bitcast i32 (i32)* %f to i8*

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-checks/sanitizer-cfi-emit-type-checks.ll
Check file: /checkout/src/test/codegen/sanitizer-cfi-emit-type-checks.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
         .
         .
         .
         .
         6: ; sanitizer_cfi_emit_type_checks::foo
         7: ; Function Attrs: nonlazybind uwtable
         8: define i32 @_ZN30sanitizer_cfi_emit_type_checks3foo17h811a4972151c5698E(i32 (i32)* noundef nonnull %f, i32 %arg) unnamed_addr #0 !type !3 {
         9: start:
        10:  %0 = bitcast i32 (i32)* %f to i8*
        11:  %1 = call i1 @llvm.type.test(i8* %0, metadata !"_ZTSFu3i32S_E")
next:11      !~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match on wrong line
        12:  br i1 %1, label %type_test.pass, label %type_test.fail
        13: 
        14: type_test.pass: ; preds = %start
        15:  %2 = call i32 %f(i32 %arg)
        16:  br label %bb1
         .
         .
>>>>>>
------------------------------------------
