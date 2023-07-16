plain
Some tests failed in compiletest suite=codegen mode=codegen host=i686-unknown-linux-gnu target=i686-unknown-linux-gnu

---- [codegen] codegen/loads.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/loads/loads.ll" "/checkout/src/test/codegen/loads.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/loads.rs:24:11: error: CHECK: expected string not found in input
// CHECK: @ptr_alignment_helper({}** {{.*}} align [[PTR_ALIGNMENT:[0-9]+]]
          ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/loads/loads.ll:1:1: note: scanning from here
; ModuleID = 'loads.a60abd95-cgu.0'
^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/loads/loads.ll:52:13: note: possible intended match here
define void @ptr_alignment_helper({}** align 4 %x) unnamed_addr #0 {
/checkout/src/test/codegen/loads.rs:31:40: error: undefined variable: PTR_ALIGNMENT
/checkout/src/test/codegen/loads.rs:31:40: error: undefined variable: PTR_ALIGNMENT
// CHECK: load i32*, i32** %x, align [[PTR_ALIGNMENT]], !nonnull !{{[0-9]+}}, !align ![[ALIGN_4_META:[0-9]+]], !noundef !{{[0-9]+}}
                                       ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/loads/loads.ll:60:7: note: possible intended match here
 %0 = load i32*, i32** %x, align 4, !nonnull !2, !align !3, !noundef !2
/checkout/src/test/codegen/loads.rs:38:68: error: undefined variable: PTR_ALIGNMENT
/checkout/src/test/codegen/loads.rs:38:68: error: undefined variable: PTR_ALIGNMENT
// CHECK: load {{%Align16|i128}}*, {{%Align16|i128}}** %x, align [[PTR_ALIGNMENT]], !nonnull !{{[0-9]+}}, !align ![[ALIGN_16_META:[0-9]+]], !noundef !{{[0-9]+}}
                                                                   ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/loads/loads.ll:67:7: note: possible intended match here
 %0 = load %Align16*, %Align16** %x, align 4, !nonnull !2, !align !4, !noundef !2
/checkout/src/test/codegen/loads.rs:45:45: error: undefined variable: PTR_ALIGNMENT
/checkout/src/test/codegen/loads.rs:45:45: error: undefined variable: PTR_ALIGNMENT
// CHECK: load i32*, i32** %{{.+}}, align [[PTR_ALIGNMENT]], !nonnull !{{[0-9]+}}, !align ![[ALIGN_4_META]], !noundef !{{[0-9]+}}
                                            ^
/checkout/src/test/codegen/loads.rs:45:94: error: undefined variable: ALIGN_4_META
// CHECK: load i32*, i32** %{{.+}}, align [[PTR_ALIGNMENT]], !nonnull !{{[0-9]+}}, !align ![[ALIGN_4_META]], !noundef !{{[0-9]+}}
                                                                                             ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/loads/loads.ll:75:7: note: possible intended match here
 %1 = load i32*, i32** %0, align 4, !nonnull !2, !align !3, !noundef !2
/checkout/src/test/codegen/loads.rs:54:40: error: undefined variable: PTR_ALIGNMENT
/checkout/src/test/codegen/loads.rs:54:40: error: undefined variable: PTR_ALIGNMENT
// CHECK: load i32*, i32** %x, align [[PTR_ALIGNMENT]]{{$}}
                                       ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/loads/loads.ll:86:7: note: possible intended match here
 %0 = load i32*, i32** %x, align 4
/checkout/src/test/codegen/loads.rs:61:40: error: undefined variable: PTR_ALIGNMENT
/checkout/src/test/codegen/loads.rs:61:40: error: undefined variable: PTR_ALIGNMENT
// CHECK: load i32*, i32** %x, align [[PTR_ALIGNMENT]], !nonnull !{{[0-9]+}}, !align ![[ALIGN_4_META]], !noundef !{{[0-9]+}}
                                       ^
/checkout/src/test/codegen/loads.rs:61:89: error: undefined variable: ALIGN_4_META
// CHECK: load i32*, i32** %x, align [[PTR_ALIGNMENT]], !nonnull !{{[0-9]+}}, !align ![[ALIGN_4_META]], !noundef !{{[0-9]+}}
                                                                                        ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/loads/loads.ll:93:7: note: possible intended match here
 %0 = load i32*, i32** %x, align 4, !nonnull !2, !align !3, !noundef !2
      ^
/checkout/src/test/codegen/loads.rs:151:18: error: undefined variable: ALIGN_4_META
// CHECK-DAG: ![[ALIGN_4_META]] = !{i64 4}
                 ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/loads/loads.ll:239:31: note: possible intended match here
 %layout = alloca { i32, i32 }, align 4

Input file: /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/loads/loads.ll
Check file: /checkout/src/test/codegen/loads.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'loads.a60abd95-cgu.0' 
check:24'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "loads.a60abd95-cgu.0" 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128" 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "i686-unknown-linux-gnu" 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:24'0     ~
            6: %Align16 = type { i128 } 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
           47: bb1: ; preds = %bb3 
check:24'0     ~~~~~~~~~~~~~~~~~~~~
           48:  ret void 
check:24'0     ~~~~~~~~~~
           49: } 
check:24'0     ~~
           50:  
check:24'0     ~
           51: ; Function Attrs: nonlazybind uwtable 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           52: define void @ptr_alignment_helper({}** align 4 %x) unnamed_addr #0 { 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:24'1                 ?                                                         possible intended match
           53: start: 
check:24'0     ~~~~~~~
           54:  ret void 
check:24'0     ~~~~~~~~~~
           55: } 
check:24'0     ~~
           56:  
check:24'0     ~
           57: ; Function Attrs: nonlazybind uwtable 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           58: define align 4 i32* @load_ref(i32** align 4 %x) unnamed_addr #0 { 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:31'0                                  X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:31'1                                                                        undefined variable: PTR_ALIGNMENT
           59: start: 
check:31'0     ~~~~~~~
           60:  %0 = load i32*, i32** %x, align 4, !nonnull !2, !align !3, !noundef !2 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:31'2           ?                                                                  possible intended match
           61:  ret i32* %0 
check:31'0     ~~~~~~~~~~~~~
           62: } 
check:31'0     ~~
           63:  
check:31'0     ~
           64: ; Function Attrs: nonlazybind uwtable 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65: define align 16 %Align16* @load_ref_higher_alignment(%Align16** align 4 %x) unnamed_addr #0 { 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:38'0                                                         X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:38'1                                                                                                    undefined variable: PTR_ALIGNMENT
           66: start: 
check:38'0     ~~~~~~~
           67:  %0 = load %Align16*, %Align16** %x, align 4, !nonnull !2, !align !4, !noundef !2 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:38'2           ?                                                                            possible intended match
           68:  ret %Align16* %0 
check:38'0     ~~~~~~~~~~~~~~~~~~
           69: } 
check:38'0     ~~
           70:  
check:38'0     ~
           71: ; Function Attrs: nonlazybind uwtable 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           72: define { i32*, i64* } @load_scalar_pair({ i32*, i64* }* align 4 %x) unnamed_addr #0 { 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:45'0                                            X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:45'1                                                                                            undefined variable: PTR_ALIGNMENT
check:45'2                                                                                            undefined variable: ALIGN_4_META
           73: start: 
check:45'0     ~~~~~~~
           74:  %0 = getelementptr inbounds { i32*, i64* }, { i32*, i64* }* %x, i32 0, i32 0 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           75:  %1 = load i32*, i32** %0, align 4, !nonnull !2, !align !3, !noundef !2 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:45'3           ?                                                                  possible intended match
           76:  %2 = getelementptr inbounds { i32*, i64* }, { i32*, i64* }* %x, i32 0, i32 1 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           77:  %3 = load i64*, i64** %2, align 4, !nonnull !2, !align !4, !noundef !2 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           78:  %4 = insertvalue { i32*, i64* } undef, i32* %1, 0 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           79:  %5 = insertvalue { i32*, i64* } %4, i64* %3, 1 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           80:  ret { i32*, i64* } %5 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~
           81: } 
check:45'0     ~~
           82:  
check:45'0     ~
           83: ; Function Attrs: nonlazybind uwtable 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           84: define i32* @load_raw_pointer(i32** align 4 %x) unnamed_addr #0 { 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:54'0                                  X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:54'1                                                                        undefined variable: PTR_ALIGNMENT
           85: start: 
check:54'0     ~~~~~~~
           86:  %0 = load i32*, i32** %x, align 4 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:54'2           ?                             possible intended match
           87:  ret i32* %0 
check:54'0     ~~~~~~~~~~~~~
           88: } 
check:54'0     ~~
           89:  
check:54'0     ~
           90: ; Function Attrs: nonlazybind uwtable 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           91: define align 4 i32* @load_box(i32** align 4 %x) unnamed_addr #0 { 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:61'0                                  X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:61'1                                                                        undefined variable: PTR_ALIGNMENT
check:61'2                                                                        undefined variable: ALIGN_4_META
           92: start: 
check:61'0     ~~~~~~~
           93:  %0 = load i32*, i32** %x, align 4, !nonnull !2, !align !3, !noundef !2 
check:61'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:61'3           ?                                                                  possible intended match
           94:  %1 = bitcast i32** %x to i32* 
check:61'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           95: ; call alloc::alloc::box_free 
check:61'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           96:  call void @_ZN5alloc5alloc8box_free17h7f77ace14fcee898E(i32* %1) 
check:61'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           97:  br label %bb1 
check:61'0     ~~~~~~~~~~~~~~~
           98:  
check:61'0     ~
            .
            .
            .
          227:  %5 = bitcast %Bytes* %1 to i8* 
          228:  %6 = bitcast %Bytes* %x to i8* 
          229:  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %5, i8* align 1 %6, i32 4, i1 false) 
          230:  %7 = bitcast %Bytes* %1 to i32* 
          231:  %8 = load i32, i32* %7, align 1 
          232:  ret i32 %8 
dag:151'0                 X error: match failed for invalid pattern
dag:151'1                   undefined variable: ALIGN_4_META
          233: } 
dag:151'0      ~~
          234:  
dag:151'0      ~
          235: ; alloc::alloc::dealloc 
dag:151'0      ~~~~~~~~~~~~~~~~~~~~~~~~
          236: ; Function Attrs: inlinehint nonlazybind uwtable 
dag:151'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          237: define internal void @_ZN5alloc5alloc7dealloc17h304c6b8718ba91aaE(i8* %ptr, i32 %0, i32 %1) unnamed_addr #1 { 
dag:151'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          238: start: 
dag:151'0      ~~~~~~~
          239:  %layout = alloca { i32, i32 }, align 4 
dag:151'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
dag:151'2                                    ?          possible intended match
          240:  %2 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %layout, i32 0, i32 0 
dag:151'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          241:  store i32 %0, i32* %2, align 4 
dag:151'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          242:  %3 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %layout, i32 0, i32 1 
dag:151'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          243:  store i32 %1, i32* %3, align 4 
dag:151'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          244: ; call core::alloc::layout::Layout::size 
dag:151'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
