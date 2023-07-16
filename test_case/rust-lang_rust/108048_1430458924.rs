plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7ea263296c702710d7fac3c6d5bccdb2895b4e79)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 404 tests
i......i.i............i....i..ii.................iii........iii.i........i.............. 88/404
.....ii...................i..........i.....i...............i.....i...iiii.......i..i.F.. 176/404
..........ii........................i.i.ii.i.i............i..i....i....i..iii.......i... 352/404
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
ii.....................iiiiiiii.i...................
failures:
failures:

---- [codegen] tests/codegen/issue-107681-unwrap_unchecked-optimized.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-107681-unwrap_unchecked-optimized/issue-107681-unwrap_unchecked-optimized.ll" "/checkout/tests/codegen/issue-107681-unwrap_unchecked-optimized.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/issue-107681-unwrap_unchecked-optimized.rs:11:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: br
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-107681-unwrap_unchecked-optimized/issue-107681-unwrap_unchecked-optimized.ll:23:2: note: found here
 br i1 %_10.i, label %bb4.split, label %bb6.split

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-107681-unwrap_unchecked-optimized/issue-107681-unwrap_unchecked-optimized.ll
Check file: /checkout/tests/codegen/issue-107681-unwrap_unchecked-optimized.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
        1: ; ModuleID = 'issue_107681_unwrap_unchecked_optimized.91c38bd7-cgu.0' 
        2: source_filename = "issue_107681_unwrap_unchecked_optimized.91c38bd7-cgu.0" 
        3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
        4: target triple = "x86_64-unknown-linux-gnu" 
        5:  
        6: %"core::panic::location::Location<'_>" = type { { [0 x i8]*, i64 }, i32, i32 } 
        7: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [2 x i64] } 
        8: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
        9:  
       10: @alloc24 = private unnamed_addr constant <{ [32 x i8] }> <{ [32 x i8] c"assertion failed: self.is_some()" }>, align 1 
       11: @alloc25 = private unnamed_addr constant <{ [66 x i8] }> <{ [66 x i8] c"/checkout/tests/codegen/issue-107681-unwrap_unchecked-optimized.rs" }>, align 1 
       12: @alloc26 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [66 x i8] }>, <{ [66 x i8] }>* @alloc25, i32 0, i32 0, i32 0), [16 x i8] c"B\00\00\00\00\00\00\00\0C\00\00\00\0E\00\00\00" }>, align 8 
       14: ; issue_107681_unwrap_unchecked_optimized::unwrap_unchecked_optimized 
       14: ; issue_107681_unwrap_unchecked_optimized::unwrap_unchecked_optimized 
       15: ; Function Attrs: nonlazybind uwtable 
       16: define noundef i32 @_ZN39issue_107681_unwrap_unchecked_optimized26unwrap_unchecked_optimized17h2afcb04876ae7100E({ i32*, i32* }* noalias nocapture noundef align 8 dereferenceable(16) %x) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
       17: start: 
       18:  %0 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %x, i64 0, i32 1 
       19:  %self1.i = load i32*, i32** %0, align 8, !alias.scope !2, !nonnull !5, !noundef !5 
       20:  %1 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %x, i64 0, i32 0 
       21:  %self2.i = load i32*, i32** %1, align 8, !alias.scope !2, !nonnull !5, !noundef !5 
       22:  %_10.i = icmp eq i32* %self1.i, %self2.i 
       23:  br i1 %_10.i, label %bb4.split, label %bb6.split 
not:11      !~                                                error: no match expected
       24:  
       25: bb4.split: ; preds = %start 
       26: ; call core::panicking::panic 
       27:  tail call void @_ZN4core9panicking5panic17haff47ae7fd4ead80E([0 x i8]* noalias noundef nonnull readonly align 1 bitcast (<{ [32 x i8] }>* @alloc24 to [0 x i8]*), i64 noundef 32, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc26 to %"core::panic::location::Location<'_>"*)) #2 
       28:  unreachable 
       29:  
       30: bb6.split: ; preds = %start 
       31:  %2 = getelementptr inbounds i32, i32* %self1.i, i64 1 
       32:  store i32* %2, i32** %0, align 8, !alias.scope !2 
       33:  %v = load i32, i32* %self1.i, align 4, !noundef !5 
       34:  ret i32 %v 
       35: } 
       36:  
       37: ; Function Attrs: nonlazybind uwtable 
       38: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, %"unwind::libunwind::_Unwind_Exception"* noundef, %"unwind::libunwind::_Unwind_Context"* noundef) unnamed_addr #0 
       40: ; core::panicking::panic 
       40: ; core::panicking::panic 
       41: ; Function Attrs: cold noinline noreturn nonlazybind uwtable 
       42: declare void @_ZN4core9panicking5panic17haff47ae7fd4ead80E([0 x i8]* noalias noundef nonnull readonly align 1, i64 noundef, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #1 
       43:  
       44: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
       45: attributes #1 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
       46: attributes #2 = { noreturn } 
       47:  
       48: !llvm.module.flags = !{!0, !1} 
       49:  
       50: !0 = !{i32 7, !"PIC Level", i32 2} 
       51: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
       52: !2 = !{!3} 
       53: !3 = distinct !{!3, !4, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h78124f2e576cceebE: %self"} 
       54: !4 = distinct !{!4, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h78124f2e576cceebE"} 
       55: !5 = !{} 
------------------------------------------



