plain
 finished in 11.837 seconds
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 350 tests
ii.....F........i...i..ii.................iiiF.......ii.i.......i.................ii.... 88/350
...i..................i...i...i.....ii..i.ii.................ii........................i 264/350
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.i.ii.i......i.......iii.......i...i.....................iiiiiiii..i..................
failures:
failures:

---- [codegen] src/test/codegen/abi-efiapi.rs#i686 stdout ----

error in revision `i686`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.i686/abi-efiapi.ll" "/checkout/src/test/codegen/abi-efiapi.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,i686"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/abi-efiapi.rs:28:9: error: i686: expected string not found in input
//i686: define dso_local void @has_efiapi
        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.i686/abi-efiapi.ll:1:1: note: scanning from here
; ModuleID = 'abi_efiapi.cb96f543-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.i686/abi-efiapi.ll:7:1: note: possible intended match here
define void @has_efiapi() unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.i686/abi-efiapi.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'abi_efiapi.cb96f543-cgu.0' 
check:28'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "abi_efiapi.cb96f543-cgu.0" 
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128" 
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "i686-unknown-linux-musl" 
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:28'0     ~
            6: ; Function Attrs: nounwind nonlazybind uwtable 
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: define void @has_efiapi() unnamed_addr #0 { 
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:28'1     ?                                            possible intended match
            8: start: 
check:28'0     ~~~~~~~
            9:  ret void 
check:28'0     ~~~~~~~~~~
           10: } 
check:28'0     ~~
           11:  
check:28'0     ~
           12: attributes #0 = { nounwind nonlazybind uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="pentium4" } 
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/avr/avr-func-addrspace.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/avr/avr-func-addrspace/avr-func-addrspace.ll" "/checkout/src/test/codegen/avr/avr-func-addrspace.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/avr/avr-func-addrspace.rs:80:11: error: CHECK: expected string not found in input
// CHECK: define void @test(){{.+}}addrspace(1)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/avr/avr-func-addrspace/avr-func-addrspace.ll:1:1: note: scanning from here
; ModuleID = 'avr_func_addrspace.55e4cf97-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/avr/avr-func-addrspace/avr-func-addrspace.ll:44:37: note: possible intended match here
 %_7 = load void (i16*, i32*) addrspace(1)*, void (i16*, i32*) addrspace(1)** bitcast (<{ i8 addrspace(1)* }>* @_ZN18avr_func_addrspace11STORAGE_FOO17h26a6d7132d01b8b5E to void (i16*, i32*) addrspace(1)**), align 1, !nonnull !0, !noundef !0

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/avr/avr-func-addrspace/avr-func-addrspace.ll
Check file: /checkout/src/test/codegen/avr/avr-func-addrspace.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'avr_func_addrspace.55e4cf97-cgu.0' 
check:80'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "avr_func_addrspace.55e4cf97-cgu.0" 
check:80'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8" 
check:80'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "avr-unknown-unknown" 
check:80'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:80'0     ~
            6: @_ZN18avr_func_addrspace11STORAGE_FOO17h26a6d7132d01b8b5E = dso_local local_unnamed_addr global <{ i8 addrspace(1)* }> <{ i8 addrspace(1)* bitcast (void (i16*, i32*) addrspace(1)* @_ZN18avr_func_addrspace19arbitrary_black_box17h67922ab850c6f054E to i8 addrspace(1)*) }>, align 1 
check:80'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
           39:  %buf = alloca i32, align 1 
check:80'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  %0 = bitcast i32* %buf to i8* 
check:80'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  call addrspace(1) void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %0) 
check:80'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42:  store i32 7, i32* %buf, align 1 
check:80'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           43:  tail call fastcc addrspace(1) void @call_through_fn_trait() 
check:80'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  %_7 = load void (i16*, i32*) addrspace(1)*, void (i16*, i32*) addrspace(1)** bitcast (<{ i8 addrspace(1)* }>* @_ZN18avr_func_addrspace11STORAGE_FOO17h26a6d7132d01b8b5E to void (i16*, i32*) addrspace(1)**), align 1, !nonnull !0, !noundef !0 
check:80'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:80'1                                         ?                                                                                                                                                                                                             possible intended match
           45:  call addrspace(1) void %_7(i16* noalias noundef readonly align 1 dereferenceable(2) bitcast (<{ [2 x i8] }>* @alloc16 to i16*), i32* noalias noundef nonnull align 1 dereferenceable(4) %buf) 
check:80'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46:  call addrspace(1) void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %0) 
check:80'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47:  ret void 
check:80'0     ~~~~~~~~~~
           48: } 
check:80'0     ~~
           49:  
check:80'0     ~
            .
            .
>>>>>>
------------------------------------------
