plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [codegen] tests/codegen/inline-hint.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inline-hint/inline-hint.ll" "/checkout/tests/codegen/inline-hint.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/inline-hint.rs:24:11: error: CHECK: expected string not found in input
// CHECK: ; <(i32, i32, i32, *const i{{16|32|64}}) as core::clone::Clone>::clone
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inline-hint/inline-hint.ll:86:38: note: scanning from here
; Function Attrs: nonlazybind uwtable

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inline-hint/inline-hint.ll
Check file: /checkout/tests/codegen/inline-hint.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
          1: ; ModuleID = 'inline_hint.e9e66dcf-cgu.0' 
          2: source_filename = "inline_hint.e9e66dcf-cgu.0" 
          3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
          4: target triple = "x86_64-unknown-linux-gnu" 
          5:  
          6: %"alloc::vec::Vec<u8>" = type { { i64, i8* }, i64 } 
          7: %A = type { %"alloc::string::String", %"alloc::string::String" } 
          8: %"alloc::string::String" = type { %"alloc::vec::Vec<u8>" } 
          9: %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>" = type { [2 x i64], i64 } 
         10: %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::None" = type {} 
         11: %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some" = type { { i8*, { i64, i64 } } } 
         12: %"alloc::alloc::Global" = type {} 
         13: %"core::ptr::metadata::PtrRepr<[u8]>" = type { [2 x i64] } 
         14: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [2 x i64] } 
         15: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
         16:  
         17: @alloc_d0afbdd8e12853f26980a9342bb4d429 = private unnamed_addr constant <{ [93 x i8] }> <{ [93 x i8] c"unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null" }>, align 1 
         18: @alloc_f87acf8c929a49cd44819755e05698cc = private unnamed_addr constant <{ [81 x i8] }> <{ [81 x i8] c"unsafe precondition(s) violated: Alignment::new_unchecked requires a power of two" }>, align 1 
         19: @0 = private unnamed_addr constant <{ [16 x i8] }> <{ [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00" }>, align 8 
         20:  
         21: ; core::ptr::drop_in_place::<alloc::vec::Vec<u8>> 
         22: ; Function Attrs: nonlazybind uwtable 
         23: define internal void @_RINvNtCs31ce1BwWT1v_4core3ptr13drop_in_placeINtNtCs5rq1iEqZpPS_5alloc3vec3VechEECsleXdIPmWM9L_11inline_hint(%"alloc::vec::Vec<u8>"* noundef %_1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
         24: start: 
         25:  %0 = alloca { i8*, i32 }, align 8 
         26: ; invoke <alloc::vec::Vec<u8> as core::ops::drop::Drop>::drop 
         27:  invoke void @_RNvXsn_NtCs5rq1iEqZpPS_5alloc3vecINtB5_3VechENtNtNtCs31ce1BwWT1v_4core3ops4drop4Drop4dropCsleXdIPmWM9L_11inline_hint(%"alloc::vec::Vec<u8>"* noalias noundef align 8 dereferenceable(24) %_1) 
         28:  to label %bb4 unwind label %cleanup 
         29:  
         30: bb3: ; preds = %cleanup 
         31:  %1 = bitcast %"alloc::vec::Vec<u8>"* %_1 to { i64, i8* }* 
         32: ; invoke core::ptr::drop_in_place::<alloc::raw_vec::RawVec<u8>> 
         33:  invoke void @_RINvNtCs31ce1BwWT1v_4core3ptr13drop_in_placeINtNtCs5rq1iEqZpPS_5alloc7raw_vec6RawVechEECsleXdIPmWM9L_11inline_hint({ i64, i8* }* noundef %1) #8 
         34:  to label %bb1 unwind label %abort 
         35:  
         36: cleanup: ; preds = %start 
         37:  %2 = landingpad { i8*, i32 } 
         38:  cleanup 
         39:  %3 = extractvalue { i8*, i32 } %2, 0 
         40:  %4 = extractvalue { i8*, i32 } %2, 1 
         41:  %5 = bitcast { i8*, i32 }* %0 to i8* 
         42:  call void @llvm.lifetime.start.p0i8(i64 16, i8* %5) 
         43:  %6 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0 
         44:  store i8* %3, i8** %6, align 8 
         45:  %7 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1 
         46:  store i32 %4, i32* %7, align 8 
         47:  br label %bb3 
         48:  
         49: bb4: ; preds = %start 
         50:  %8 = bitcast %"alloc::vec::Vec<u8>"* %_1 to { i64, i8* }* 
         51: ; call core::ptr::drop_in_place::<alloc::raw_vec::RawVec<u8>> 
         52:  call void @_RINvNtCs31ce1BwWT1v_4core3ptr13drop_in_placeINtNtCs5rq1iEqZpPS_5alloc7raw_vec6RawVechEECsleXdIPmWM9L_11inline_hint({ i64, i8* }* noundef %8) 
         53:  ret void 
         54:  
         55: abort: ; preds = %bb3 
         56:  %9 = landingpad { i8*, i32 } 
         57:  cleanup 
         58:  %10 = extractvalue { i8*, i32 } %9, 0 
         59:  %11 = extractvalue { i8*, i32 } %9, 1 
         60: ; call core::panicking::panic_cannot_unwind 
         61:  call void @_ZN4core9panicking19panic_cannot_unwind17h428ee4579b7cd52cE() #9 
         62:  unreachable 
         63:  
         64: bb1: ; preds = %bb3 
         65:  %12 = bitcast { i8*, i32 }* %0 to i8** 
         66:  %13 = load i8*, i8** %12, align 8, !noundef !2 
         67:  %14 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1 
         68:  %15 = load i32, i32* %14, align 8, !noundef !2 
         69:  %16 = bitcast { i8*, i32 }* %0 to i8* 
         70:  call void @llvm.lifetime.end.p0i8(i64 16, i8* %16) 
         71:  %17 = insertvalue { i8*, i32 } undef, i8* %13, 0 
         72:  %18 = insertvalue { i8*, i32 } %17, i32 %15, 1 
         73:  resume { i8*, i32 } %18 
         74: } 
         75:  
         76: ; core::ptr::drop_in_place::<alloc::raw_vec::RawVec<u8>> 
         77: ; Function Attrs: nonlazybind uwtable 
         78: define internal void @_RINvNtCs31ce1BwWT1v_4core3ptr13drop_in_placeINtNtCs5rq1iEqZpPS_5alloc7raw_vec6RawVechEECsleXdIPmWM9L_11inline_hint({ i64, i8* }* noundef %_1) unnamed_addr #0 { 
         79: start: 
         80: ; call <alloc::raw_vec::RawVec<u8> as core::ops::drop::Drop>::drop 
         81:  call void @_RNvXs1_NtCs5rq1iEqZpPS_5alloc7raw_vecINtB5_6RawVechENtNtNtCs31ce1BwWT1v_4core3ops4drop4Drop4dropCsleXdIPmWM9L_11inline_hint({ i64, i8* }* noalias noundef align 8 dereferenceable(16) %_1) 
         82:  ret void 
         83: } 
         84:  
         85: ; core::ptr::drop_in_place::<inline_hint::A> 
         86: ; Function Attrs: nonlazybind uwtable 
check:24                                          X error: no match found
         87: define internal void @_RINvNtCs31ce1BwWT1v_4core3ptr13drop_in_placeNtCsleXdIPmWM9L_11inline_hint1AEBI_(%A* noundef %_1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
         88: start: 
check:24     ~~~~~~~
check:24     ~~~~~~~
         89:  %0 = alloca { i8*, i32 }, align 8 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         90:  %1 = bitcast %A* %_1 to %"alloc::string::String"* 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         91: ; invoke core::ptr::drop_in_place::<alloc::string::String> 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         92:  invoke void @_RINvNtCs31ce1BwWT1v_4core3ptr13drop_in_placeNtNtCs5rq1iEqZpPS_5alloc6string6StringECsleXdIPmWM9L_11inline_hint(%"alloc::string::String"* noundef %1) 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         93:  to label %bb4 unwind label %cleanup 
         94:  
check:24     ~
check:24     ~
         95: bb3: ; preds = %cleanup 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~
         96:  %2 = getelementptr inbounds %A, %A* %_1, i32 0, i32 1 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         97: ; invoke core::ptr::drop_in_place::<alloc::string::String> 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         98:  invoke void @_RINvNtCs31ce1BwWT1v_4core3ptr13drop_in_placeNtNtCs5rq1iEqZpPS_5alloc6string6StringECsleXdIPmWM9L_11inline_hint(%"alloc::string::String"* noundef %2) #8 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         99:  to label %bb1 unwind label %abort 
        100:  
check:24     ~
check:24     ~
        101: cleanup: ; preds = %start 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~
        102:  %3 = landingpad { i8*, i32 } 
        103:  cleanup 
check:24     ~~~~~~~~~
check:24     ~~~~~~~~~
        104:  %4 = extractvalue { i8*, i32 } %3, 0 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        105:  %5 = extractvalue { i8*, i32 } %3, 1 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        106:  %6 = bitcast { i8*, i32 }* %0 to i8* 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        107:  call void @llvm.lifetime.start.p0i8(i64 16, i8* %6) 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        108:  %7 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        109:  store i8* %4, i8** %7, align 8 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        110:  %8 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        111:  store i32 %5, i32* %8, align 8 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        112:  br label %bb3 
        113:  
check:24     ~
check:24     ~
        114: bb4: ; preds = %start 
check:24     ~~~~~~~~~~~~~~~~~~~~~~
        115:  %9 = getelementptr inbounds %A, %A* %_1, i32 0, i32 1 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        116: ; call core::ptr::drop_in_place::<alloc::string::String> 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        117:  call void @_RINvNtCs31ce1BwWT1v_4core3ptr13drop_in_placeNtNtCs5rq1iEqZpPS_5alloc6string6StringECsleXdIPmWM9L_11inline_hint(%"alloc::string::String"* noundef %9) 
        118:  ret void 
check:24     ~~~~~~~~~~
        119:  
check:24     ~
check:24     ~
        120: abort: ; preds = %bb3 
check:24     ~~~~~~~~~~~~~~~~~~~~~~
        121:  %10 = landingpad { i8*, i32 } 
        122:  cleanup 
check:24     ~~~~~~~~~
check:24     ~~~~~~~~~
        123:  %11 = extractvalue { i8*, i32 } %10, 0 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        124:  %12 = extractvalue { i8*, i32 } %10, 1 
        125: ; call core::panicking::panic_cannot_unwind 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        126:  call void @_ZN4core9panicking19panic_cannot_unwind17h428ee4579b7cd52cE() #9 
        127:  unreachable 
check:24     ~~~~~~~~~~~~~
        128:  
check:24     ~
check:24     ~
        129: bb1: ; preds = %bb3 
check:24     ~~~~~~~~~~~~~~~~~~~~
        130:  %13 = bitcast { i8*, i32 }* %0 to i8** 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        131:  %14 = load i8*, i8** %13, align 8, !noundef !2 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        132:  %15 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        133:  %16 = load i32, i32* %15, align 8, !noundef !2 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        134:  %17 = bitcast { i8*, i32 }* %0 to i8* 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        135:  call void @llvm.lifetime.end.p0i8(i64 16, i8* %17) 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        136:  %18 = insertvalue { i8*, i32 } undef, i8* %14, 0 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        137:  %19 = insertvalue { i8*, i32 } %18, i32 %16, 1 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        138:  resume { i8*, i32 } %19 
        139: } 
check:24     ~~
        140:  
check:24     ~
check:24     ~
        141: ; core::ptr::drop_in_place::<alloc::string::String> 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        142: ; Function Attrs: nonlazybind uwtable 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        143: define internal void @_RINvNtCs31ce1BwWT1v_4core3ptr13drop_in_placeNtNtCs5rq1iEqZpPS_5alloc6string6StringECsleXdIPmWM9L_11inline_hint(%"alloc::string::String"* noundef %_1) unnamed_addr #0 { 
        144: start: 
check:24     ~~~~~~~
check:24     ~~~~~~~
        145:  %0 = bitcast %"alloc::string::String"* %_1 to %"alloc::vec::Vec<u8>"* 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        146: ; call core::ptr::drop_in_place::<alloc::vec::Vec<u8>> 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        147:  call void @_RINvNtCs31ce1BwWT1v_4core3ptr13drop_in_placeINtNtCs5rq1iEqZpPS_5alloc3vec3VechEECsleXdIPmWM9L_11inline_hint(%"alloc::vec::Vec<u8>"* noundef %0) 
        148:  ret void 
check:24     ~~~~~~~~~~
        149: } 
check:24     ~~
check:24     ~~
        150:  
check:24     ~
        151: ; <core::ptr::non_null::NonNull<u8>>::new_unchecked 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        152: ; Function Attrs: inlinehint nonlazybind uwtable 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        153: define internal noundef nonnull i8* @_RNvMs1_NtNtCs31ce1BwWT1v_4core3ptr8non_nullINtB5_7NonNullhE13new_uncheckedCsleXdIPmWM9L_11inline_hint(i8* noundef %ptr) unnamed_addr #1 { 
        154: start: 
check:24     ~~~~~~~
check:24     ~~~~~~~
        155:  %0 = alloca i64, align 8 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~
        156:  %_9 = alloca i8*, align 8 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
        157:  %_2 = alloca i8*, align 8 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
        158:  %1 = alloca i8*, align 8 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~
        159:  %2 = bitcast i8** %_2 to i8* 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        160:  call void @llvm.lifetime.start.p0i8(i64 8, i8* %2) 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        161:  store i8* %ptr, i8** %_2, align 8 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        162:  %ptr1 = load i8*, i8** %_2, align 8, !noundef !2 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        163:  %3 = bitcast i8** %_9 to i8* 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        164:  call void @llvm.lifetime.start.p0i8(i64 8, i8* %3) 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        165:  store i8* %ptr1, i8** %_9, align 8 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        166:  %ptr2 = load i8*, i8** %_9, align 8, !noundef !2 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        167:  %_13 = bitcast i8* %ptr2 to {}* 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        168:  %4 = bitcast i64* %0 to i8* 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        169:  call void @llvm.lifetime.start.p0i8(i64 8, i8* %4) 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        170:  %5 = bitcast i64* %0 to {}** 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        171:  store {}* %_13, {}** %5, align 8 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        172:  %_12 = load i64, i64* %0, align 8, !noundef !2 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        173:  %6 = bitcast i64* %0 to i8* 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        174:  call void @llvm.lifetime.end.p0i8(i64 8, i8* %6) 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        175:  %_7 = icmp eq i64 %_12, 0 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
        176:  %7 = bitcast i8** %_9 to i8* 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        177:  call void @llvm.lifetime.end.p0i8(i64 8, i8* %7) 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        178:  %_6 = xor i1 %_7, true 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~
        179:  %_5 = xor i1 %_6, true 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~
        180:  br i1 %_5, label %bb1, label %bb2 
        181:  
check:24     ~
check:24     ~
        182: bb2: ; preds = %start 
check:24     ~~~~~~~~~~~~~~~~~~~~~~
        183:  %8 = bitcast i8** %_2 to i8* 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        184:  call void @llvm.lifetime.end.p0i8(i64 8, i8* %8) 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        185:  store i8* %ptr, i8** %1, align 8 
check:24     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        186:  %9 = load i8*, i8** %1, align 8, !nonnull !2, !noundef !2 
          .
          .
          .
>>>>>>
>>>>>>
------------------------------------------


---- [codegen] tests/codegen/local-generics-in-exe-internalized.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/local-generics-in-exe-internalized/local-generics-in-exe-internalized.ll" "/checkout/tests/codegen/local-generics-in-exe-internalized.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/local-generics-in-exe-internalized.rs:5:17: error: CHECK-LABEL: expected string not found in input
// CHECK-LABEL: ; local_generics_in_exe_internalized::foo
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/local-generics-in-exe-internalized/local-generics-in-exe-internalized.ll:1:1: note: scanning from here
; ModuleID = 'local_generics_in_exe_internalized.f498e780-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/local-generics-in-exe-internalized/local-generics-in-exe-internalized.ll:1:12: note: possible intended match here
; ModuleID = 'local_generics_in_exe_internalized.f498e780-cgu.0'

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/local-generics-in-exe-internalized/local-generics-in-exe-internalized.ll
Check file: /checkout/tests/codegen/local-generics-in-exe-internalized.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'local_generics_in_exe_internalized.f498e780-cgu.0' 
label:5'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
label:5'1                ?                                                      possible intended match
           2: source_filename = "local_generics_in_exe_internalized.f498e780-cgu.0" 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "x86_64-unknown-linux-gnu" 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
label:5'0     ~
           6: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [2 x i64] } 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           7: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           8:  
label:5'0     ~
           9: @vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, i8*, i8* }> <{ i8* bitcast (void (i64**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h17b4f72b5b9c24bfE" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i32 (i64**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h8df4654a3f8d77e4E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h411fb9fbed05b272E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h411fb9fbed05b272E" to i8*) }>, align 8 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          10:  
label:5'0     ~
          11: ; std::sys_common::backtrace::__rust_begin_short_backtrace 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          12: ; Function Attrs: noinline nonlazybind uwtable 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          13: define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hc8da87fe8335d7d8E(void ()* noundef nonnull %f) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          14: start: 
label:5'0     ~~~~~~~
          15:  %0 = alloca { i8*, i32 }, align 8 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          16: ; call core::ops::function::FnOnce::call_once 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          17:  call void @_ZN4core3ops8function6FnOnce9call_once17h2f9f77fc5ac118d2E(void ()* noundef nonnull %f) 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          18:  call void asm sideeffect "", "~{memory}"(), !srcloc !3 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          19:  br label %bb4 
label:5'0     ~~~~~~~~~~~~~~~
          20:  
label:5'0     ~
          21: bb4: ; preds = %start 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~
          22:  ret void 
label:5'0     ~~~~~~~~~~
          23:  
label:5'0     ~
          24: bb2: ; No predecessors! 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~
          25:  %1 = bitcast { i8*, i32 }* %0 to i8** 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          26:  %2 = load i8*, i8** %1, align 8, !noundef !4 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          27:  %3 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          28:  %4 = load i32, i32* %3, align 8, !noundef !4 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          29:  %5 = bitcast { i8*, i32 }* %0 to i8* 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          30:  call void @llvm.lifetime.end.p0i8(i64 16, i8* %5) 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          31:  %6 = insertvalue { i8*, i32 } undef, i8* %2, 0 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          32:  %7 = insertvalue { i8*, i32 } %6, i32 %4, 1 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          33:  resume { i8*, i32 } %7 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~
          34: } 
label:5'0     ~~
          35:  
label:5'0     ~
          36: ; std::rt::lang_start 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~
          37: ; Function Attrs: nonlazybind uwtable 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          38: define hidden noundef i64 @_ZN3std2rt10lang_start17h564bf129e489a3edE(void ()* noundef nonnull %main, i64 noundef %argc, i8** noundef %argv, i8 noundef %sigpipe) unnamed_addr #1 { 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          39: start: 
label:5'0     ~~~~~~~
          40:  %_8 = alloca i64*, align 8 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          41:  %_5 = alloca i64, align 8 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
          42:  %0 = bitcast i64* %_5 to i8* 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          43:  call void @llvm.lifetime.start.p0i8(i64 8, i8* %0) 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          44:  %1 = bitcast i64** %_8 to i8* 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          45:  call void @llvm.lifetime.start.p0i8(i64 8, i8* %1) 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          46:  %2 = bitcast i64** %_8 to void ()** 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          47:  store void ()* %main, void ()** %2, align 8 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          48:  %_6.0 = bitcast i64** %_8 to {}* 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          49: ; call std::rt::lang_start_internal 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          50:  %3 = call noundef i64 @_ZN3std2rt19lang_start_internal17he44378c70e14838eE({}* noundef nonnull align 1 %_6.0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8], i8*, i8*, i8* }>* @vtable.0 to [3 x i64]*), i64 noundef %argc, i8** noundef %argv, i8 noundef %sigpipe) 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          51:  store i64 %3, i64* %_5, align 8 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          52:  %4 = load i64, i64* %_5, align 8, !noundef !4 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          53:  %5 = bitcast i64** %_8 to i8* 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          54:  call void @llvm.lifetime.end.p0i8(i64 8, i8* %5) 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          55:  %6 = bitcast i64* %_5 to i8* 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          56:  call void @llvm.lifetime.end.p0i8(i64 8, i8* %6) 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          57:  ret i64 %4 
label:5'0     ~~~~~~~~~~~~
          58: } 
label:5'0     ~~
          59:  
label:5'0     ~
          60: ; std::rt::lang_start::{{closure}} 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          61: ; Function Attrs: inlinehint nonlazybind uwtable 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          62: define internal noundef i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h411fb9fbed05b272E"(i64** noalias noundef readonly align 8 dereferenceable(8) %_1) unnamed_addr #2 { 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          63: start: 
label:5'0     ~~~~~~~
          64:  %self = alloca i8, align 1 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          65:  call void @llvm.lifetime.start.p0i8(i64 1, i8* %self) 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          66:  %0 = bitcast i64** %_1 to void ()** 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          67:  %_4 = load void ()*, void ()** %0, align 8, !nonnull !4, !noundef !4 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          68: ; call std::sys_common::backtrace::__rust_begin_short_backtrace 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          69:  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hc8da87fe8335d7d8E(void ()* noundef nonnull %_4) 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          70: ; call <() as std::process::Termination>::report 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          71:  %1 = call noundef i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6bafa24716019bf7E"() 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          72:  store i8 %1, i8* %self, align 1 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          73:  %_6 = load i8, i8* %self, align 1, !noundef !4 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          74:  %2 = zext i8 %_6 to i32 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          75:  call void @llvm.lifetime.end.p0i8(i64 1, i8* %self) 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          76:  ret i32 %2 
label:5'0     ~~~~~~~~~~~~
          77: } 
label:5'0     ~~
          78:  
label:5'0     ~
          79: ; core::ops::function::FnOnce::call_once{{vtable.shim}} 
label:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
