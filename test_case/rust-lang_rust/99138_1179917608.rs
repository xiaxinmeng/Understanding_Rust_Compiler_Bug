
---- [codegen] src/test/codegen/mem-replace-direct-memcpy.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/home/ltdk/rust/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/home/ltdk/rust/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll" "/home/ltdk/rust/src/test/codegen/mem-replace-direct-memcpy.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/home/ltdk/rust/src/test/codegen/mem-replace-direct-memcpy.rs:20:11: error: CHECK: expected string not found in input
// CHECK: call void @llvm.memcpy.{{.+}}({{i8\*|ptr}} align 1 %{{.*}}, {{i8\*|ptr}} align 1 %dest, i{{.*}} 1, i1 false)
          ^
/home/ltdk/rust/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:52:21: note: scanning from here
; core::mem::replace
                    ^
/home/ltdk/rust/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:138:2: note: possible intended match here
 call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %_4, i8* align 1 %src, i64 1, i1 false)
 ^

Input file: /home/ltdk/rust/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll
Check file: /home/ltdk/rust/src/test/codegen/mem-replace-direct-memcpy.rs

-dump-input=help explains the following input dump.

Input was:
<<<<<<
            .
            .
            .
           47: define internal i8 @"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$10into_inner17h35f74d8c84d7bcc5E"(i8 %slot) unnamed_addr #0 {
           48: start:
           49:  ret i8 %slot
           50: }
           51:
           52: ; core::mem::replace
check:20'0                         X error: no match found
           53: ; Function Attrs: inlinehint nonlazybind uwtable
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54: define internal i8 @_ZN4core3mem7replace17hbec7fbe13492cafeE(i8* noalias noundef align 1 dereferenceable(1) %dest, i8 %src) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55: start:
check:20'0     ~~~~~~~
           56:  %0 = alloca { i8*, i32 }, align 8
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           57:  %_7 = alloca i8, align 1
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
          133: ; call core::mem::maybe_uninit::MaybeUninit<T>::as_mut_ptr
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          134:  %_4 = call i8* @"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h4bc92292b08ff7a1E"(i8* noalias noundef align 1 dereferenceable(1) %tmp)
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          135:  br label %bb2
check:20'0     ~~~~~~~~~~~~~~~
          136:
check:20'0     ~
          137: bb2: ; preds = %bb1
check:20'0     ~~~~~~~~~~~~~~~~~~~~
          138:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %_4, i8* align 1 %src, i64 1, i1 false)
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:20'1      ?                                                                                         possible intended match
          139:  %_6 = load i8, i8* %tmp, align 1
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          140: ; call core::mem::maybe_uninit::MaybeUninit<T>::assume_init
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          141:  %1 = call i8 @"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$11assume_init17ha7013b104b271e83E"(i8 %_6, %"core::panic::location::Location"* noalias noundef readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc21 to %"core::panic::location::Location"*))
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          142:  br label %bb3
check:20'0     ~~~~~~~~~~~~~~~
          143:
check:20'0     ~
            .
            .
            .
>>>>>>
------------------------------------------



failures:
    [codegen] src/test/codegen/mem-replace-direct-memcpy.rs
