plain
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 354 tests
ii.............i....i..ii.................iii........ii.i.......i.................ii.... 88/354
......F......i............i..i.................i...iii........i..i......iii.ii........i. 176/354
.ii..F.i...................i..i.....i...ii...iii.................ii..................... 264/354
...i.i.ii.i......i....F..iii.......i...i.....................iiiiiiii.i..............F.. 352/354
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [codegen] src/test/codegen/external-no-mangle-fns.rs stdout ----


error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-fns/external-no-mangle-fns.ll" "/checkout/src/test/codegen/external-no-mangle-fns.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/external-no-mangle-fns.rs:38:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: define internal
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-fns/external-no-mangle-fns.ll:255:18: note: scanning from here
; Function Attrs: noinline nonlazybind uwtable
                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-fns/external-no-mangle-fns.ll:256:1: note: possible intended match here
define void @_ZN22external_no_mangle_fns1x17h3b52ebf22067e371E() unnamed_addr #2 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/external-no-mangle-fns/external-no-mangle-fns.ll
Check file: /checkout/src/test/codegen/external-no-mangle-fns.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
         250: start: 
         251:  ret void 
         252: } 
         253:  
         254: ; external_no_mangle_fns::x 
         255: ; Function Attrs: noinline nonlazybind uwtable 
next:38'0                      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
         256: define void @_ZN22external_no_mangle_fns1x17h3b52ebf22067e371E() unnamed_addr #2 { 
next:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
next:38'1     ?                                                                                   possible intended match
         257: start: 
next:38'0     ~~~~~~~
         258:  %0 = alloca i32, align 4 
next:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
         259:  %_5 = alloca i32*, align 8 
next:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         260:  %1 = bitcast i32** %_5 to i8* 
next:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         261:  call void @llvm.lifetime.start.p0i8(i64 8, i8* %1) 
next:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/link-dead-code.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/link-dead-code/link-dead-code.ll" "/checkout/src/test/codegen/link-dead-code.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/link-dead-code.rs:10:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: define hidden
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/link-dead-code/link-dead-code.ll:7:18: note: scanning from here
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn
                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/link-dead-code/link-dead-code.ll:8:1: note: possible intended match here
define i32 @_ZN14link_dead_code8const_fn17ha98dc9de82d7225cE() unnamed_addr #0 {
^
/checkout/src/test/codegen/link-dead-code.rs:21:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: define hidden
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/link-dead-code/link-dead-code.ll:21:18: note: scanning from here
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn
                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/link-dead-code/link-dead-code.ll:22:1: note: possible intended match here
define i32 @_ZN14link_dead_code10private_fn17hce5fdadfd9890a1bE() unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/link-dead-code/link-dead-code.ll
Check file: /checkout/src/test/codegen/link-dead-code.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'link_dead_code.8b95b121-cgu.0' 
           2: source_filename = "link_dead_code.8b95b121-cgu.0" 
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
           4: target triple = "x86_64-unknown-linux-gnu" 
           6: ; link_dead_code::const_fn 
           6: ; link_dead_code::const_fn 
           7: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
next:10'0                      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           8: define i32 @_ZN14link_dead_code8const_fn17ha98dc9de82d7225cE() unnamed_addr #0 { 
next:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
next:10'1     ?                                                                                 possible intended match
           9: start: 
next:10'0     ~~~~~~~
          10:  ret i32 1 
next:10'0     ~~~~~~~~~~~
          11: } 
next:10'0     ~~
          12:  
next:10'0     ~
          13: ; link_dead_code::inline_fn 
next:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
          14: ; Function Attrs: inlinehint mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
          15: define hidden i32 @_ZN14link_dead_code9inline_fn17h40c8ae5d8dd63d23E() unnamed_addr #1 { 
          17:  ret i32 2 
          18: } 
          19:  
          20: ; link_dead_code::private_fn 
          20: ; link_dead_code::private_fn 
          21: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
next:21'0                      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
          22: define i32 @_ZN14link_dead_code10private_fn17hce5fdadfd9890a1bE() unnamed_addr #0 { 
next:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
next:21'1     ?                                                                                    possible intended match
          23: start: 
next:21'0     ~~~~~~~
          24:  ret i32 3 
next:21'0     ~~~~~~~~~~~
          25: } 
next:21'0     ~~
          26:  
next:21'0     ~
          27: attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
next:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/some-global-nonnull.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/some-global-nonnull/some-global-nonnull.ll" "/checkout/src/test/codegen/some-global-nonnull.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/some-global-nonnull.rs:6:16: error: CHECK-NEXT: is not on the line after the previous match
// CHECK-NEXT: start:
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/some-global-nonnull/some-global-nonnull.ll:11:1: note: 'next' match was here
^
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/some-global-nonnull/some-global-nonnull.ll:6:6: note: previous match ended here
@test = unnamed_addr alias void (), void ()* @_ZN19some_global_nonnull6inner017h9c8dd77415a58585E
     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/some-global-nonnull/some-global-nonnull.ll:7:1: note: non-matching line after previous match is here
^


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/some-global-nonnull/some-global-nonnull.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
        6: @test = unnamed_addr alias void (), void ()* @_ZN19some_global_nonnull6inner017h9c8dd77415a58585E 
        7:  
        8: ; some_global_nonnull::test_inner 
        9: ; Function Attrs: nonlazybind uwtable 
       10: define void @_ZN19some_global_nonnull10test_inner17hcdf1062aa00be493E(i64* %0) unnamed_addr #0 { 
       11: start: 
next:6     !~~~~~  error: match on wrong line
       12:  %.not = icmp eq i64* %0, null 
       13:  br i1 %.not, label %bb3, label %bb1 
       14:  
       15: bb1: ; preds = %start 
       16:  %1 = bitcast i64* %0 to void ()* 
        .
        .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/thread-local.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll" "/checkout/src/test/codegen/thread-local.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/thread-local.rs:17:11: error: CHECK: expected string not found in input
// CHECK: [[TLS:@.+]] = internal thread_local unnamed_addr global
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:9:111: note: scanning from here
@_ZN16thread_local_aux1A7__getit3VAL17h150aaf5c477fbca5E = external thread_local local_unnamed_addr global i64
                                                                                                              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:10:45: note: possible intended match here
@_ZN12thread_local1A7__getit3VAL17h6bf9be1eebdbc01aE = thread_local local_unnamed_addr global <{ [4 x i8] }> <{ [4 x i8] c"\01\00\00\00" }>, align 4
/checkout/src/test/codegen/thread-local.rs:22:35: error: undefined variable: TLS
/checkout/src/test/codegen/thread-local.rs:22:35: error: undefined variable: TLS
 // CHECK: %0 = load i32, {{.*}}[[TLS]]{{.*}}
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:16:2: note: possible intended match here
 %0 = load i32, i32* bitcast (<{ [4 x i8] }>* @_ZN12thread_local1A7__getit3VAL17h6bf9be1eebdbc01aE to i32*), align 4
/checkout/src/test/codegen/thread-local.rs:30:34: error: undefined variable: TLS
/checkout/src/test/codegen/thread-local.rs:30:34: error: undefined variable: TLS
 // CHECK: store i32 %0, {{.*}}[[TLS]]{{.*}}
                                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:23:2: note: possible intended match here
 store i32 %0, i32* bitcast (<{ [4 x i8] }>* @_ZN12thread_local1A7__getit3VAL17h6bf9be1eebdbc01aE to i32*), align 4, !alias.scope !2, !noalias !5

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll
Check file: /checkout/src/test/codegen/thread-local.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'thread_local.f5a81537-cgu.0' 
            2: source_filename = "thread_local.f5a81537-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] } 
            7: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
            8:  
            9: @_ZN16thread_local_aux1A7__getit3VAL17h150aaf5c477fbca5E = external thread_local local_unnamed_addr global i64 
check:17'0                                                                                                                   X error: no match found
           10: @_ZN12thread_local1A7__getit3VAL17h6bf9be1eebdbc01aE = thread_local local_unnamed_addr global <{ [4 x i8] }> <{ [4 x i8] c"\01\00\00\00" }>, align 4 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:17'1                                                 ?                                                                                                         possible intended match
           11: @_ZN12thread_local1A7__getit5STATE17hec7af2efbc79bb8eE = thread_local local_unnamed_addr global <{ [1 x i8] }> zeroinitializer, align 1 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           12:  
check:17'0     ~
           13: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readonly uwtable willreturn 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14: define i32 @get() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
check:17'0     ~~~~~~~~~~~~~~~
check:22'0                    X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:22'1                                                                                                                                                                                  undefined variable: TLS
           15: start: 
check:22'0     ~~~~~~~
           16:  %0 = load i32, i32* bitcast (<{ [4 x i8] }>* @_ZN12thread_local1A7__getit3VAL17h6bf9be1eebdbc01aE to i32*), align 4 
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:22'2      ?                                                                                                                    possible intended match
           17:  ret i32 %0 
check:22'0     ~~~~~~~~~~~~
           18: } 
check:22'0     ~~
           19:  
check:22'0     ~
           20: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind uwtable willreturn 
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21: define void @set(i32 %0) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
check:22'0     ~~~~~~~~~~~~~~~~
check:30'0                     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:30'1                                                                                                                                                                                         undefined variable: TLS
           22: start: 
check:30'0     ~~~~~~~
           23:  store i32 %0, i32* bitcast (<{ [4 x i8] }>* @_ZN12thread_local1A7__getit3VAL17h6bf9be1eebdbc01aE to i32*), align 4, !alias.scope !2, !noalias !5 
check:30'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:30'2      ?                                                                                                                                                 possible intended match
           24:  ret void 
check:30'0     ~~~~~~~~~~
           25: } 
check:30'0     ~~
           26:  
check:30'0     ~
           27: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readonly uwtable willreturn 
check:30'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28: define i64 @get_aux() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
check:30'0     ~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
