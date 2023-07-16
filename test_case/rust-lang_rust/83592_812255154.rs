plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=wasm32-unknown-emscripten

---- [codegen] codegen/abi-x86-interrupt.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-x86-interrupt/abi-x86-interrupt.ll" "/checkout/src/test/codegen/abi-x86-interrupt.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/abi-x86-interrupt.rs:14:11: error: CHECK: expected string not found in input
// CHECK: define x86_intrcc i64 @has_x86_interrupt_abi
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-x86-interrupt/abi-x86-interrupt.ll:1:1: note: scanning from here
; ModuleID = 'abi_x86_interrupt.3a1fbbbh-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-x86-interrupt/abi-x86-interrupt.ll:7:11: note: possible intended match here
define dso_local x86_intrcc i64 @has_x86_interrupt_abi(i64* noalias nocapture byval(i64) dereferenceable(8) %a) unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-x86-interrupt/abi-x86-interrupt.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'abi_x86_interrupt.3a1fbbbh-cgu.0'
check:14'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "abi_x86_interrupt.3a1fbbbh-cgu.0"
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "wasm32-unknown-emscripten"
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5: 
check:14'0     ~
            6: ; Function Attrs: nounwind uwtable
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: define dso_local x86_intrcc i64 @has_x86_interrupt_abi(i64* noalias nocapture byval(i64) dereferenceable(8) %a) unnamed_addr #0 {
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:14'1               ?                                                                                                                       possible intended match
            8: start:
check:14'0     ~~~~~~
            9:  %_2 = load i64, i64* %a, align 8
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           10:  %0 = mul i64 %_2, 2
check:14'0     ~~~~~~~~~~~~~~~~~~~~
           11:  ret i64 %0
check:14'0     ~~~~~~~~~~~
           12: }
check:14'0     ~
            .
            .
>>>>>>


------------------------------------------


---- [codegen] codegen/abi-sysv64.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-sysv64/abi-sysv64.ll" "/checkout/src/test/codegen/abi-sysv64.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/abi-sysv64.rs:13:11: error: CHECK: expected string not found in input
// CHECK: define x86_64_sysvcc i64 @has_sysv64_abi
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-sysv64/abi-sysv64.ll:1:1: note: scanning from here
; ModuleID = 'abi_sysv64.3a1fbbbh-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-sysv64/abi-sysv64.ll:7:11: note: possible intended match here
define dso_local x86_64_sysvcc i64 @has_sysv64_abi(i64 %a) unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-sysv64/abi-sysv64.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'abi_sysv64.3a1fbbbh-cgu.0'
check:13'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "abi_sysv64.3a1fbbbh-cgu.0"
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "wasm32-unknown-emscripten"
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5: 
check:13'0     ~
            6: ; Function Attrs: nounwind uwtable
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: define dso_local x86_64_sysvcc i64 @has_sysv64_abi(i64 %a) unnamed_addr #0 {
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:13'1               ?                                                                  possible intended match
            8: start:
check:13'0     ~~~~~~
            9:  %0 = mul i64 %a, 2
check:13'0     ~~~~~~~~~~~~~~~~~~~
           10:  ret i64 %0
check:13'0     ~~~~~~~~~~~
           11: }
check:13'0     ~
           12: 
check:13'0     ~
           13: attributes #0 = { nounwind uwtable "target-cpu"="generic" }
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
>>>>>>
------------------------------------------


---- [codegen] codegen/abi-repr-ext.rs stdout ----
---- [codegen] codegen/abi-repr-ext.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-repr-ext/abi-repr-ext.ll" "/checkout/src/test/codegen/abi-repr-ext.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/abi-repr-ext.rs:9:11: error: CHECK: expected string not found in input
// CHECK: define signext i8 @test()
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-repr-ext/abi-repr-ext.ll:1:1: note: scanning from here
; ModuleID = 'abi_repr_ext.3a1fbbbh-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-repr-ext/abi-repr-ext.ll:7:11: note: possible intended match here
define dso_local signext i8 @test() unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-repr-ext/abi-repr-ext.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'abi_repr_ext.3a1fbbbh-cgu.0'
check:9'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "abi_repr_ext.3a1fbbbh-cgu.0"
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "wasm32-unknown-emscripten"
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5: 
check:9'0     ~
           6: ; Function Attrs: norecurse nounwind readnone uwtable willreturn
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           7: define dso_local signext i8 @test() unnamed_addr #0 {
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:9'1               ?                                           possible intended match
           8: start:
check:9'0     ~~~~~~
           9:  ret i8 0
check:9'0     ~~~~~~~~~
          10: }
check:9'0     ~
          11: 
check:9'0     ~
          12: attributes #0 = { norecurse nounwind readnone uwtable willreturn "target-cpu"="generic" }
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
>>>>>>
------------------------------------------


---- [codegen] codegen/dealloc-no-unwind.rs stdout ----
---- [codegen] codegen/dealloc-no-unwind.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/dealloc-no-unwind/dealloc-no-unwind.ll" "/checkout/src/test/codegen/dealloc-no-unwind.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/dealloc-no-unwind.rs:18:18: error: CHECK-LABEL: expected string not found in input
 // CHECK-LABEL: define void @a
                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/dealloc-no-unwind/dealloc-no-unwind.ll:1:1: note: scanning from here
; ModuleID = 'dealloc_no_unwind.3a1fbbbh-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/dealloc-no-unwind/dealloc-no-unwind.ll:10:1: note: possible intended match here
define hidden void @"_ZN62_$LT$dealloc_no_unwind..A$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0fe5fd3493436881E"(%A* noalias nocapture nonnull readnone align 1 %self) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/dealloc-no-unwind/dealloc-no-unwind.ll
Check file: /checkout/src/test/codegen/dealloc-no-unwind.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'dealloc_no_unwind.3a1fbbbh-cgu.0'
label:18'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "dealloc_no_unwind.3a1fbbbh-cgu.0"
label:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
label:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "wasm32-unknown-emscripten"
label:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5: 
label:18'0     ~
            6: %A = type {}
label:18'0     ~~~~~~~~~~~~
            7: 
label:18'0     ~
            8: ; <dealloc_no_unwind::A as core::ops::drop::Drop>::drop
label:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            9: ; Function Attrs: nounwind uwtable
label:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           10: define hidden void @"_ZN62_$LT$dealloc_no_unwind..A$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0fe5fd3493436881E"(%A* noalias nocapture nonnull readnone align 1 %self) unnamed_addr #0 {
label:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
label:18'1     ?                                                                                                                                                                                          possible intended match
           11: start:
label:18'0     ~~~~~~
           12:  tail call void @foo()
label:18'0     ~~~~~~~~~~~~~~~~~~~~~~
           13:  ret void
label:18'0     ~~~~~~~~~
           14: }
label:18'0     ~
           15: 
label:18'0     ~
            .
            .
>>>>>>


------------------------------------------


---- [codegen] codegen/fewer-names.rs#NO stdout ----

error in revision `NO`: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fewer-names.NO/fewer-names.ll" "/checkout/src/test/codegen/fewer-names.rs" "--check-prefixes" "CHECK,NO"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/fewer-names.rs:14:14: error: NO-LABEL: expected string not found in input
// NO-LABEL: define i32 @sum(i32 %x, i32 %y)
             ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fewer-names.NO/fewer-names.ll:1:1: note: scanning from here
; ModuleID = 'fewer_names.3a1fbbbh-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fewer-names.NO/fewer-names.ll:7:11: note: possible intended match here
define dso_local i32 @sum(i32 %x, i32 %y) unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fewer-names.NO/fewer-names.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'fewer_names.3a1fbbbh-cgu.0'
label:14'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "fewer_names.3a1fbbbh-cgu.0"
label:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
label:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "wasm32-unknown-emscripten"
label:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5: 
label:14'0     ~
            6: ; Function Attrs: norecurse nounwind readnone uwtable willreturn
label:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: define dso_local i32 @sum(i32 %x, i32 %y) unnamed_addr #0 {
label:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
label:14'1               ?                                                 possible intended match
            8: start:
label:14'0     ~~~~~~
            9:  %z = add i32 %y, %x
label:14'0     ~~~~~~~~~~~~~~~~~~~~
           10:  ret i32 %z
label:14'0     ~~~~~~~~~~~
           11: }
label:14'0     ~
           12: 
label:14'0     ~
           13: attributes #0 = { norecurse nounwind readnone uwtable willreturn "target-cpu"="generic" }
label:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
>>>>>>
------------------------------------------


---- [codegen] codegen/external-no-mangle-fns.rs stdout ----
---- [codegen] codegen/external-no-mangle-fns.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-fns/external-no-mangle-fns.ll" "/checkout/src/test/codegen/external-no-mangle-fns.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/external-no-mangle-fns.rs:7:11: error: CHECK: expected string not found in input
// CHECK: define void @a()
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-fns/external-no-mangle-fns.ll:1:1: note: scanning from here
; ModuleID = 'external_no_mangle_fns.3a1fbbbh-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-fns/external-no-mangle-fns.ll:15:1: note: possible intended match here
define internal zeroext i1 @_ZN4core10intrinsics23is_aligned_and_not_null17hf16794a7d24730d4E(i32* %ptr) unnamed_addr #0 {
^
/checkout/src/test/codegen/external-no-mangle-fns.rs:41:12: error: CHECK: expected string not found in input
 // CHECK: define void @g()
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-fns/external-no-mangle-fns.ll:206:17: note: scanning from here
define internal void @_ZN22external_no_mangle_fns1x17h8253d251c96911a4E() unnamed_addr #3 {
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-fns/external-no-mangle-fns.ll:217:11: note: possible intended match here
define dso_local void @g() unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-fns/external-no-mangle-fns.ll
Check file: /checkout/src/test/codegen/external-no-mangle-fns.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'external_no_mangle_fns.3a1fbbbh-cgu.0'
check:7'0      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "external_no_mangle_fns.3a1fbbbh-cgu.0"
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "wasm32-unknown-emscripten"
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5: 
check:7'0      ~
            6: %"core::panic::Location" = type { [0 x i32], { [0 x i8]*, i32 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
           10: @str.0 = internal constant [57 x i8] c"attempt to calculate the remainder with a divisor of zero"
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11: @alloc2 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"*\00\00\00" }>, align 4
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           12: 
check:7'0      ~
           13: ; core::intrinsics::is_aligned_and_not_null
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14: ; Function Attrs: uwtable
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~
           15: define internal zeroext i1 @_ZN4core10intrinsics23is_aligned_and_not_null17hf16794a7d24730d4E(i32* %ptr) unnamed_addr #0 {
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:7'1      ?                                                                                                                          possible intended match
           16: start:
check:7'0      ~~~~~~
           17:  %0 = alloca i8, align 1
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~
           18: ; call core::ptr::const_ptr::<impl *const T>::is_null
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19:  %_3 = call zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17h2b3ee82388ab2a30E"(i32* %ptr)
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           20:  br label %bb5
check:7'0      ~~~~~~~~~~~~~~
            .
            .
          201:  ret void
          201:  ret void
check:7'0      ~~~~~~~~~
          202: }
check:7'0      ~
          203: 
check:7'0      ~
          204: ; external_no_mangle_fns::x
check:7'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~
          205: ; Function Attrs: noinline uwtable
          206: define internal void @_ZN22external_no_mangle_fns1x17h8253d251c96911a4E() unnamed_addr #3 {
check:41'0                     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
          207: start:
check:41'0     ~~~~~~
          208: ; call core::ptr::read_volatile
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          209:  %_1 = call i32 @_ZN4core3ptr13read_volatile17h47b9b9a3522686feE(i32* bitcast (<{ [4 x i8] }>* @alloc2 to i32*))
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          210:  br label %bb1
check:41'0     ~~~~~~~~~~~~~~
          211: 
check:41'0     ~
          212: bb1: ; preds = %start
check:41'0     ~~~~~~~~~~~~~~~~~~~~~
          213:  ret void
check:41'0     ~~~~~~~~~
          214: }
check:41'0     ~
          215: 
check:41'0     ~
          216: ; Function Attrs: uwtable
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          217: define dso_local void @g() unnamed_addr #0 {
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:41'1               ?                                  possible intended match
          218: start:
check:41'0     ~~~~~~
          219: ; call external_no_mangle_fns::x
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          220:  call void @_ZN22external_no_mangle_fns1x17h8253d251c96911a4E()
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          221:  br label %bb1
check:41'0     ~~~~~~~~~~~~~~
          222: 
check:41'0     ~
            .
            .
>>>>>>


------------------------------------------


---- [codegen] codegen/ffi-returns-twice.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/ffi-returns-twice/ffi-returns-twice.ll" "/checkout/src/test/codegen/ffi-returns-twice.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/ffi-returns-twice.rs:8:18: error: CHECK-LABEL: expected string not found in input
 // CHECK-LABEL: declare void @foo()
                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/ffi-returns-twice/ffi-returns-twice.ll:1:1: note: scanning from here
; ModuleID = 'ffi_returns_twice.3a1fbbbh-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/ffi-returns-twice/ffi-returns-twice.ll:10:2: note: possible intended match here
 call void @foo()


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/ffi-returns-twice/ffi-returns-twice.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'ffi_returns_twice.3a1fbbbh-cgu.0'
label:8'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "ffi_returns_twice.3a1fbbbh-cgu.0"
label:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
label:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "wasm32-unknown-emscripten"
label:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5: 
label:8'0     ~
           6: ; ffi_returns_twice::bar
label:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           7: ; Function Attrs: uwtable
label:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           8: define hidden void @_ZN17ffi_returns_twice3bar17hcd38d711a1d03585E() unnamed_addr #0 {
label:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           9: start:
label:8'0     ~~~~~~
          10:  call void @foo()
label:8'0     ~~~~~~~~~~~~~~~~~
label:8'1      ?                possible intended match
          11:  br label %bb1
label:8'0     ~~~~~~~~~~~~~~
          12: 
label:8'0     ~
          13: bb1: ; preds = %start
label:8'0     ~~~~~~~~~~~~~~~~~~~~~
          14:  ret void
label:8'0     ~~~~~~~~~
          15: }
label:8'0     ~
           .
           .
>>>>>>
---
test result: FAILED. 169 passed; 23 failed; 72 ignored; 0 measured; 0 filtered out; finished in 1.39s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-wasm32-unknown-emscripten" "--suite" "codegen" "--mode" "codegen" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/latest/bin/node" "--npm" "/emsdk-portable/node/latest/bin/npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 --host= --target wasm32-unknown-emscripten --exclude library/alloc
Build completed unsuccessfully in 0:24:57
