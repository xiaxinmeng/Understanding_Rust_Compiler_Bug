
Check compiletest suite=codegen mode=codegen (aarch64-apple-darwin -> aarch64-apple-darwin)

running 350 tests
iiiiiiiiii...ii......i................iii.iiiii.....i........i...................i...... 88/350
........ii..iii.......i...................i............i........i....i......ii......i..i 176/350
..................i...i.i.i....iiii..iii....iiiiiii.iiiiiii....F....................i... 264/350
..i......ii......iii.......i.........................ii.....i.i.......................
failures:

---- [codegen] src/test/codegen/mem-replace-direct-memcpy.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/Users/nathan/rust/rust/build/aarch64-apple-darwin/ci-llvm/bin/FileCheck" "--input-file" "/Users/nathan/rust/rust/build/aarch64-apple-darwin/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll" "/Users/nathan/rust/rust/src/test/codegen/mem-replace-direct-memcpy.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/Users/nathan/rust/rust/src/test/codegen/mem-replace-direct-memcpy.rs:20:11: error: CHECK: expected string not found in input
// CHECK: call void @llvm.memcpy.{{.+}}({{i8\*|ptr}} align 1 %{{.*}}, {{i8\*|ptr}} align 1 %dest, i{{.*}} 1, i1 false)
          ^
/Users/nathan/rust/rust/build/aarch64-apple-darwin/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:47:21: note: scanning from here
; core::mem::replace
                    ^
/Users/nathan/rust/rust/build/aarch64-apple-darwin/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:129:2: note: possible intended match here
 call void @llvm.memcpy.p0.p0.i64(ptr align 1 %_4, ptr align 1 %src, i64 1, i1 false)
 ^

Input file: /Users/nathan/rust/rust/build/aarch64-apple-darwin/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll
Check file: /Users/nathan/rust/rust/src/test/codegen/mem-replace-direct-memcpy.rs

-dump-input=help explains the following input dump.

Input was:
<<<<<<
            .
            .
            .
           42: define internal i8 @"_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$10into_inner17h98f1424f9d2c8ff5E"(i8 %slot) unnamed_addr #0 { 
           43: start: 
           44:  ret i8 %slot 
           45: } 
           46:  
           47: ; core::mem::replace 
check:20'0                         X error: no match found
           48: ; Function Attrs: inlinehint uwtable 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           49: define internal i8 @_ZN4core3mem7replace17hd07fab279ae0e11dE(ptr noalias noundef align 1 dereferenceable(1) %dest, i8 %src) unnamed_addr #1 personality ptr @rust_eh_personality { 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50: start: 
check:20'0     ~~~~~~~
           51:  %0 = alloca { ptr, i32 }, align 8 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           52:  %_7 = alloca i8, align 1 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
          124: ; call core::mem::maybe_uninit::MaybeUninit<T>::as_mut_ptr 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          125:  %_4 = call ptr @"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h25c1c862ec6279c9E"(ptr noalias noundef align 1 dereferenceable(1) %tmp) 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          126:  br label %bb2 
check:20'0     ~~~~~~~~~~~~~~~
          127:  
check:20'0     ~
          128: bb2: ; preds = %bb1 
check:20'0     ~~~~~~~~~~~~~~~~~~~~
          129:  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %_4, ptr align 1 %src, i64 1, i1 false) 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:20'1      ?                                                                                     possible intended match
          130:  %_6 = load i8, ptr %tmp, align 1 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          131: ; call core::mem::maybe_uninit::MaybeUninit<T>::assume_init 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          132:  %1 = call i8 @"_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$11assume_init17hbd6e80e342715b18E"(i8 %_6, ptr noalias noundef readonly align 8 dereferenceable(24) @alloc21) 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          133:  br label %bb3 
check:20'0     ~~~~~~~~~~~~~~~
          134:  
check:20'0     ~
            .
            .
            .
>>>>>>
------------------------------------------



failures:
    [codegen] src/test/codegen/mem-replace-direct-memcpy.rs

test result: FAILED. 274 passed; 1 failed; 75 ignored; 0 measured; 0 filtered out; finished in 2.25s

Some tests failed in compiletest suite=codegen mode=codegen host=aarch64-apple-darwin target=aarch64-apple-darwin
