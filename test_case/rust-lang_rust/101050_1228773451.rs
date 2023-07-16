plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: x86_64-gnu-llvm-12
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-12
---

running 347 tests
ii...i..........i...ii..ii.................iii........ii.i.......i.................ii... 88/347
..............i..............i................i...iii........i..i......i.........i...... 176/347
.i..................i....i..i.F...ii..i.ii.................ii........................i.i 264/347
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [codegen] src/test/codegen/mem-replace-direct-memcpy.rs stdout ----
---- [codegen] src/test/codegen/mem-replace-direct-memcpy.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll" "/checkout/src/test/codegen/mem-replace-direct-memcpy.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/mem-replace-direct-memcpy.rs:20:11: error: CHECK: expected string not found in input
// CHECK: call void @llvm.memcpy.{{.+}}({{i8\*|ptr}} align 1 %{{.*}}, {{i8\*|ptr}} align 1 %dest, i{{.*}} 1, i1 false)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:53:1: note: scanning from here
; Function Attrs: inlinehint nonlazybind uwtable
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:138:2: note: possible intended match here
 call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %_4, i8* align 1 %src, i64 1, i1 false)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll
Check file: /checkout/src/test/codegen/mem-replace-direct-memcpy.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           48: start:
           49:  ret i8 %slot
           50: }
           52: ; core::mem::replace
           52: ; core::mem::replace
           53: ; Function Attrs: inlinehint nonlazybind uwtable
check:20'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           54: define internal i8 @_ZN4core3mem7replace17h12dc1e5910550ca0E(i8* noalias noundef align 1 dereferenceable(1) %dest, i8 %src) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55: start:
check:20'0     ~~~~~~
           56:  %0 = alloca { i8*, i32 }, align 8
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           57:  %_7 = alloca i8, align 1
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           58:  store i8 1, i8* %_7, align 1
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
          133: ; call core::mem::maybe_uninit::MaybeUninit<T>::as_mut_ptr
          133: ; call core::mem::maybe_uninit::MaybeUninit<T>::as_mut_ptr
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          134:  %_4 = call i8* @"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h2529020c56141b3bE"(i8* noalias noundef align 1 dereferenceable(1) %tmp)
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          135:  br label %bb2
check:20'0     ~~~~~~~~~~~~~~
          136: 
check:20'0     ~
          137: bb2: ; preds = %bb1
check:20'0     ~~~~~~~~~~~~~~~~~~~
          138:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %_4, i8* align 1 %src, i64 1, i1 false)
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:20'1      ?                                                                                        possible intended match
          139:  %_6 = load i8, i8* %tmp, align 1
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          140: ; call core::mem::maybe_uninit::MaybeUninit<T>::assume_init
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          141:  %1 = call i8 @"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$11assume_init17hebfb7cc5885e0a50E"(i8 %_6, %"core::panic::location::Location"* noalias noundef readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc21 to %"core::panic::location::Location"*))
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          142:  br label %bb3
check:20'0     ~~~~~~~~~~~~~~
          143: 
check:20'0     ~
            .
            .
>>>>>>
------------------------------------------
