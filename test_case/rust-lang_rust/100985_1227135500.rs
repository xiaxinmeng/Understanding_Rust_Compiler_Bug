plain
test [ui] src/test/ui/rfc-2497-if-let-chains/irrefutable-lets.rs#disallowed ... ok
test [ui] src/test/ui/rfc-2091-track-caller/std-panic-locations.rs#mir-opt ... ok
test [ui] src/test/ui/rfc-2306/convert-id-const-with-gate.rs ... ok
test [ui] src/test/ui/rfc-2565-param-attrs/attr-without-param.rs ... ok
test [ui] src/test/ui/rfc-2627-raw-dylib/import-name-type-invalid-format.rs ... ignored
test [ui] src/test/ui/rfc-2627-raw-dylib/import-name-type-multiple.rs ... ignored
test [ui] src/test/ui/rfc-2627-raw-dylib/import-name-type-unknown-value.rs ... ignored
test [ui] src/test/ui/rfc-2627-raw-dylib/import-name-type-unsupported-link-kind.rs ... ignored
test [ui] src/test/ui/rfc-2627-raw-dylib/import-name-type-x86-only.rs ... ignored
test [ui] src/test/ui/rfc-2565-param-attrs/param-attrs-builtin-attrs.rs ... ok
test [ui] src/test/ui/rfc-2565-param-attrs/param-attrs-2018.rs ... ok
test [ui] src/test/ui/rfc-2627-raw-dylib/link-ordinal-multiple.rs ... ignored
test [ui] src/test/ui/rfc-2497-if-let-chains/no-double-assigments.rs ... ok
---
failures:

---- [codegen] src/test/codegen/alloc-optimisation.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation/alloc-optimisation.ll" "/checkout/src/test/codegen/alloc-optimisation.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/alloc-optimisation.rs:10:17: error: CHECK-NEXT: is not on the line after the previous match
 // CHECK-NEXT: ret void
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation/alloc-optimisation.ll:13:2: note: 'next' match was here
 ret void
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation/alloc-optimisation.ll:8:7: note: previous match ended here
      ^
      ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation/alloc-optimisation.ll:9:1: note: non-matching line after previous match is here
; call core::mem::valid_align::ValidAlign::new_unchecked::runtime

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation/alloc-optimisation.ll
Check file: /checkout/src/test/codegen/alloc-optimisation.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
         .
         .
         .
         .
         8: start: 
         9: ; call core::mem::valid_align::ValidAlign::new_unchecked::runtime 
        10:  tail call void @_ZN4core3mem11valid_align10ValidAlign13new_unchecked7runtime17h1ef0a6d76a69e379E(i64 4) 
        11: ; call core::mem::valid_align::ValidAlign::new_unchecked::runtime 
        12:  tail call void @_ZN4core3mem11valid_align10ValidAlign13new_unchecked7runtime17h1ef0a6d76a69e379E(i64 4) 
        13:  ret void 
next:10      !~~~~~~~  error: match on wrong line
        14: } 
        15:  
        16: ; Function Attrs: nonlazybind uwtable 
        17: declare noundef i32 @rust_eh_personality(i32, i32 noundef, i64, ptr, ptr) unnamed_addr #0 
         .
         .
         .
>>>>>>
>>>>>>
------------------------------------------


---- [codegen] src/test/codegen/issue-96497-slice-size-nowrap.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-96497-slice-size-nowrap/issue-96497-slice-size-nowrap.ll" "/checkout/src/test/codegen/issue-96497-slice-size-nowrap.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/issue-96497-slice-size-nowrap.rs:27:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: store i32 42
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-96497-slice-size-nowrap/issue-96497-slice-size-nowrap.ll:49:2: note: found here
 store i32 42, ptr %3, align 4

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-96497-slice-size-nowrap/issue-96497-slice-size-nowrap.ll
Check file: /checkout/src/test/codegen/issue-96497-slice-size-nowrap.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
       44:  %_4 = icmp ugt i64 %1, 1 
       45:  br i1 %_4, label %"_ZN4core3ptr59drop_in_place$LT$alloc..boxed..Box$LT$$u5b$u32$u5d$$GT$$GT$17he789d453960d96d1E.exit", label %panic, !prof !3 
       46:  
       47: "_ZN4core3ptr59drop_in_place$LT$alloc..boxed..Box$LT$$u5b$u32$u5d$$GT$$GT$17he789d453960d96d1E.exit": ; preds = %start 
       48:  %3 = getelementptr inbounds [0 x i32], ptr %0, i64 0, i64 1 
       49:  store i32 42, ptr %3, align 4 
not:27      !~~~~~~~~~~~                   error: no match expected
       50: ; call core::mem::valid_align::ValidAlign::new_unchecked::runtime 
       51:  tail call void @_ZN4core3mem11valid_align10ValidAlign13new_unchecked7runtime17h1ef0a6d76a69e379E(i64 4) 
       52:  %4 = shl nsw i64 %1, 2 
       53:  tail call void @__rust_dealloc(ptr nonnull %0, i64 %4, i64 4) #5 
       54:  ret void 
        .
        .
>>>>>>
------------------------------------------
