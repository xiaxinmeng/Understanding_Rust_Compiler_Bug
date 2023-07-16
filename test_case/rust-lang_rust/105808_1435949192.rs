plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
failures:

---- [codegen] tests/codegen/slice-windows-no-bounds-check.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-windows-no-bounds-check/slice-windows-no-bounds-check.ll" "/checkout/tests/codegen/slice-windows-no-bounds-check.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/slice-windows-no-bounds-check.rs:13:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-windows-no-bounds-check/slice-windows-no-bounds-check.ll:35:125: note: found here
 tail call void @_ZN4core5slice5index26slice_start_index_len_fail17h97aba30d51dc1f1eE(i64 noundef 1, i64 noundef 0, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc46 to %"core::panic::location::Location<'_>"*)) #4, !noalias !2
                                                                                                                            ^~~~~
/checkout/tests/codegen/slice-windows-no-bounds-check.rs:14:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: fail
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-windows-no-bounds-check/slice-windows-no-bounds-check.ll:34:50: note: found here
; call core::slice::index::slice_start_index_len_fail
                                                 ^~~~
/checkout/tests/codegen/slice-windows-no-bounds-check.rs:24:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-windows-no-bounds-check/slice-windows-no-bounds-check.ll:79:125: note: found here
 tail call void @_ZN4core5slice5index26slice_start_index_len_fail17h97aba30d51dc1f1eE(i64 noundef 1, i64 noundef 0, %"core::panic::location::Location<'_>"* noalias noundef nonnull readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc46 to %"core::panic::location::Location<'_>"*)) #4, !noalias !27
                                                                                                                            ^~~~~
/checkout/tests/codegen/slice-windows-no-bounds-check.rs:25:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: fail
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-windows-no-bounds-check/slice-windows-no-bounds-check.ll:78:50: note: found here
; call core::slice::index::slice_start_index_len_fail
                                                 ^~~~
/checkout/tests/codegen/slice-windows-no-bounds-check.rs:32:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-windows-no-bounds-check/slice-windows-no-bounds-check.ll:116:130: note: found here
 tail call void @_ZN4core5slice5index24slice_end_index_len_fail17h21e90144e0668f52E(i64 noundef %_19.0.i, i64 noundef 0, %"core::panic::location::Location<'_>"* noalias noundef nonnull readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc25 to %"core::panic::location::Location<'_>"*)) #4, !noalias !33
                                                                                                                                 ^~~~~
/checkout/tests/codegen/slice-windows-no-bounds-check.rs:33:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: fail
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-windows-no-bounds-check/slice-windows-no-bounds-check.ll:115:48: note: found here
; call core::slice::index::slice_end_index_len_fail

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-windows-no-bounds-check/slice-windows-no-bounds-check.ll
Check file: /checkout/tests/codegen/slice-windows-no-bounds-check.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
        1: ; ModuleID = 'slice_windows_no_bounds_check.8bba4728-cgu.0' 
        2: source_filename = "slice_windows_no_bounds_check.8bba4728-cgu.0" 
        3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
        4: target triple = "x86_64-unknown-linux-gnu" 
        5:  
        6: %"core::panic::location::Location<'_>" = type { { [0 x i8]*, i64 }, i32, i32 } 
        7: %"core::slice::iter::Windows<'_, u32>" = type { { [0 x i32]*, i64 }, i64 } 
        8: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [2 x i64] } 
        9: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
       10:  
       11: @alloc45 = private unnamed_addr constant <{ [40 x i8] }> <{ [40 x i8] c"/checkout/library/core/src/slice/iter.rs" }>, align 1 
       12: @alloc25 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @alloc45, i32 0, i32 0, i32 0), [16 x i8] c"(\00\00\00\00\00\00\00h\05\00\00\17\00\00\00" }>, align 8 
       13: @alloc46 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @alloc45, i32 0, i32 0, i32 0), [16 x i8] c"(\00\00\00\00\00\00\00,\05\00\00\17\00\00\00" }>, align 8 
       14:  
       15: ; Function Attrs: nonlazybind uwtable 
       16: define { i64, i64 } @naive_string_search([0 x i8]* noalias noundef nonnull readonly align 1 %haystack.0, i64 noundef %haystack.1, [0 x i8]* noalias noundef nonnull readonly align 1 %0, i64 noundef %1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
       17: start: 
       18:  %2 = icmp eq i64 %1, 0 
       19:  br i1 %2, label %bb3, label %bb12 
       20:  
       21: bb12: ; preds = %start 
       22:  %_11.i.i.i.i.i = getelementptr [0 x i8], [0 x i8]* %0, i64 0, i64 0 
       23:  %_2.i.i29 = icmp ult i64 %haystack.1, %1 
       24:  br i1 %_2.i.i29, label %bb3, label %"_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17hb7a2fd9a0d79dd98E.exit.i.i" 
       25:  
       26: "_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17hb7a2fd9a0d79dd98E.exit.i.i": ; preds = %bb12, %"_ZN4core4iter6traits8iterator8Iterator8position5check28_$u7b$$u7b$closure$u7d$$u7d$17h22cf92db040beb04E.exit.i" 
       27:  %accum.0.i32 = phi i64 [ %.ph.i.i, %"_ZN4core4iter6traits8iterator8Iterator8position5check28_$u7b$$u7b$closure$u7d$$u7d$17h22cf92db040beb04E.exit.i" ], [ 0, %bb12 ] 
       28:  %_13.0.i.i.i4.i31 = phi i64 [ %_13.0.i.i.i.i, %"_ZN4core4iter6traits8iterator8Iterator8position5check28_$u7b$$u7b$closure$u7d$$u7d$17h22cf92db040beb04E.exit.i" ], [ %haystack.1, %bb12 ] 
       29:  %_4.sroa.0.030 = phi [0 x i8]* [ %5, %"_ZN4core4iter6traits8iterator8Iterator8position5check28_$u7b$$u7b$closure$u7d$$u7d$17h22cf92db040beb04E.exit.i" ], [ %haystack.0, %bb12 ] 
       30:  %_3.i.i.i = icmp eq i64 %_13.0.i.i.i4.i31, 0 
       31:  br i1 %_3.i.i.i, label %bb1.i.i.i, label %"_ZN29slice_windows_no_bounds_check19naive_string_search28_$u7b$$u7b$closure$u7d$$u7d$17h7972d1e00e5230e9E.exit.i.i" 
       32:  
       33: bb1.i.i.i: ; preds = %"_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17hb7a2fd9a0d79dd98E.exit.i.i" 
       34: ; call core::slice::index::slice_start_index_len_fail 
not:14                                                      !~~~  error: no match expected
       35:  tail call void @_ZN4core5slice5index26slice_start_index_len_fail17h97aba30d51dc1f1eE(i64 noundef 1, i64 noundef 0, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc46 to %"core::panic::location::Location<'_>"*)) #4, !noalias !2 
not:13                                                                                                                                 !~~~~                                                                                                                                                                                      error: no match expected
       36:  unreachable 
       37:  
       38: "_ZN29slice_windows_no_bounds_check19naive_string_search28_$u7b$$u7b$closure$u7d$$u7d$17h7972d1e00e5230e9E.exit.i.i": ; preds = %"_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17hb7a2fd9a0d79dd98E.exit.i.i" 
       39:  %_9.i.i.i.i.i = getelementptr [0 x i8], [0 x i8]* %_4.sroa.0.030, i64 0, i64 0 
       40:  %bcmp.i.i.i.i.i = tail call i32 @bcmp(i8* nonnull %_9.i.i.i.i.i, i8* nonnull %_11.i.i.i.i.i, i64 %1) #5, !alias.scope !10, !noalias !17 
       41:  %3 = icmp eq i32 %bcmp.i.i.i.i.i, 0 
       42:  br i1 %3, label %bb3, label %"_ZN4core4iter6traits8iterator8Iterator8position5check28_$u7b$$u7b$closure$u7d$$u7d$17h22cf92db040beb04E.exit.i" 
       43:  
       44: "_ZN4core4iter6traits8iterator8Iterator8position5check28_$u7b$$u7b$closure$u7d$$u7d$17h22cf92db040beb04E.exit.i": ; preds = %"_ZN29slice_windows_no_bounds_check19naive_string_search28_$u7b$$u7b$closure$u7d$$u7d$17h7972d1e00e5230e9E.exit.i.i" 
       45:  %4 = getelementptr inbounds [0 x i8], [0 x i8]* %_4.sroa.0.030, i64 0, i64 1 
       46:  %5 = bitcast i8* %4 to [0 x i8]* 
       47:  %_13.0.i.i.i.i = add i64 %_13.0.i.i.i4.i31, -1 
       48:  %.ph.i.i = add i64 %accum.0.i32, 1 
       49:  %_2.i.i = icmp ult i64 %_13.0.i.i.i.i, %1 
       50:  br i1 %_2.i.i, label %bb3, label %"_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17hb7a2fd9a0d79dd98E.exit.i.i" 
       51:  
       52: bb3: ; preds = %"_ZN4core4iter6traits8iterator8Iterator8position5check28_$u7b$$u7b$closure$u7d$$u7d$17h22cf92db040beb04E.exit.i", %"_ZN29slice_windows_no_bounds_check19naive_string_search28_$u7b$$u7b$closure$u7d$$u7d$17h7972d1e00e5230e9E.exit.i.i", %bb12, %start 
       53:  %.sroa.4.1 = phi i64 [ 0, %start ], [ 0, %bb12 ], [ %.ph.i.i, %"_ZN4core4iter6traits8iterator8Iterator8position5check28_$u7b$$u7b$closure$u7d$$u7d$17h22cf92db040beb04E.exit.i" ], [ %accum.0.i32, %"_ZN29slice_windows_no_bounds_check19naive_string_search28_$u7b$$u7b$closure$u7d$$u7d$17h7972d1e00e5230e9E.exit.i.i" ] 
       54:  %.sroa.0.1 = phi i64 [ 1, %start ], [ 0, %bb12 ], [ 0, %"_ZN4core4iter6traits8iterator8Iterator8position5check28_$u7b$$u7b$closure$u7d$$u7d$17h22cf92db040beb04E.exit.i" ], [ 1, %"_ZN29slice_windows_no_bounds_check19naive_string_search28_$u7b$$u7b$closure$u7d$$u7d$17h7972d1e00e5230e9E.exit.i.i" ] 
       55:  %6 = insertvalue { i64, i64 } undef, i64 %.sroa.0.1, 0 
       56:  %7 = insertvalue { i64, i64 } %6, i64 %.sroa.4.1, 1 
       57:  ret { i64, i64 } %7 
       58: } 
       59:  
       60: ; Function Attrs: nonlazybind uwtable 
       61: define { i32*, i64 } @next(%"core::slice::iter::Windows<'_, u32>"* noalias nocapture noundef align 8 dereferenceable(24) %w) unnamed_addr #0 { 
       62: start: 
       63:  tail call void @llvm.experimental.noalias.scope.decl(metadata !22) 
       64:  %0 = getelementptr inbounds %"core::slice::iter::Windows<'_, u32>", %"core::slice::iter::Windows<'_, u32>"* %w, i64 0, i32 1 
       65:  %_3.i = load i64, i64* %0, align 8, !alias.scope !22, !noundef !25 
       66:  %1 = getelementptr inbounds %"core::slice::iter::Windows<'_, u32>", %"core::slice::iter::Windows<'_, u32>"* %w, i64 0, i32 0, i32 1 
       67:  %_14.1.i = load i64, i64* %1, align 8, !alias.scope !22, !noundef !25 
       68:  %_2.i = icmp ugt i64 %_3.i, %_14.1.i 
       69:  br i1 %_2.i, label %"_ZN94_$LT$core..slice..iter..Windows$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0caf6eb78e720133E.exit", label %"_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h811abdf3acb74a66E.exit.i" 
       70:  
       71: "_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h811abdf3acb74a66E.exit.i": ; preds = %start 
       72:  %2 = getelementptr inbounds %"core::slice::iter::Windows<'_, u32>", %"core::slice::iter::Windows<'_, u32>"* %w, i64 0, i32 0, i32 0 
       73:  %self.0.i = load [0 x i32]*, [0 x i32]** %2, align 8, !alias.scope !22, !nonnull !25, !align !26 
       74:  %_3.i.i = icmp eq i64 %_14.1.i, 0 
       75:  br i1 %_3.i.i, label %bb1.i.i, label %"_ZN110_$LT$core..ops..range..RangeFrom$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17ha866c018f633b262E.exit.i" 
       76:  
       77: bb1.i.i: ; preds = %"_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h811abdf3acb74a66E.exit.i" 
       78: ; call core::slice::index::slice_start_index_len_fail 
not:25                                                      !~~~  error: no match expected
       79:  tail call void @_ZN4core5slice5index26slice_start_index_len_fail17h97aba30d51dc1f1eE(i64 noundef 1, i64 noundef 0, %"core::panic::location::Location<'_>"* noalias noundef nonnull readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc46 to %"core::panic::location::Location<'_>"*)) #4, !noalias !27 
not:24                                                                                                                                 !~~~~                                                                                                                                                                                               error: no match expected
       80:  unreachable 
       81:  
       82: "_ZN110_$LT$core..ops..range..RangeFrom$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17ha866c018f633b262E.exit.i": ; preds = %"_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h811abdf3acb74a66E.exit.i" 
       83:  %3 = getelementptr [0 x i32], [0 x i32]* %self.0.i, i64 0, i64 0 
       84:  %4 = getelementptr inbounds [0 x i32], [0 x i32]* %self.0.i, i64 0, i64 1 
       85:  %_13.0.i.i.i = add i64 %_14.1.i, -1 
       86:  %5 = bitcast %"core::slice::iter::Windows<'_, u32>"* %w to i32** 
       87:  store i32* %4, i32** %5, align 8, !alias.scope !22 
       88:  store i64 %_13.0.i.i.i, i64* %1, align 8, !alias.scope !22 
       89:  br label %"_ZN94_$LT$core..slice..iter..Windows$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0caf6eb78e720133E.exit" 
       90:  
       91: "_ZN94_$LT$core..slice..iter..Windows$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0caf6eb78e720133E.exit": ; preds = %start, %"_ZN110_$LT$core..ops..range..RangeFrom$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17ha866c018f633b262E.exit.i" 
       92:  %.sroa.0.0.i = phi i32* [ %3, %"_ZN110_$LT$core..ops..range..RangeFrom$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17ha866c018f633b262E.exit.i" ], [ null, %start ] 
       93:  %6 = insertvalue { i32*, i64 } undef, i32* %.sroa.0.0.i, 0 
       94:  %7 = insertvalue { i32*, i64 } %6, i64 %_3.i, 1 
       95:  ret { i32*, i64 } %7 
       96: } 
       97:  
       98: ; Function Attrs: nonlazybind uwtable 
       99: define { i32*, i64 } @next_back(%"core::slice::iter::Windows<'_, u32>"* noalias nocapture noundef align 8 dereferenceable(24) %w) unnamed_addr #0 { 
      100: start: 
      101:  tail call void @llvm.experimental.noalias.scope.decl(metadata !30) 
      102:  %0 = getelementptr inbounds %"core::slice::iter::Windows<'_, u32>", %"core::slice::iter::Windows<'_, u32>"* %w, i64 0, i32 1 
      103:  %_3.i = load i64, i64* %0, align 8, !alias.scope !30, !noundef !25 
      104:  %1 = getelementptr inbounds %"core::slice::iter::Windows<'_, u32>", %"core::slice::iter::Windows<'_, u32>"* %w, i64 0, i32 0, i32 1 
      105:  %_20.1.i = load i64, i64* %1, align 8, !alias.scope !30, !noundef !25 
      106:  %_2.i = icmp ult i64 %_20.1.i, %_3.i 
      107:  br i1 %_2.i, label %"_ZN109_$LT$core..slice..iter..Windows$LT$T$GT$$u20$as$u20$core..iter..traits..double_ended..DoubleEndedIterator$GT$9next_back17hb0e8283b5a9894bbE.exit", label %bb2.i 
      108:  
      109: bb2.i: ; preds = %start 
      110:  %_19.0.i = add i64 %_20.1.i, -1 
      111:  %_9.i.i = icmp eq i64 %_20.1.i, 0 
      112:  br i1 %_9.i.i, label %bb3.i.i, label %"_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h811abdf3acb74a66E.exit.i" 
      113:  
      114: bb3.i.i: ; preds = %bb2.i 
      115: ; call core::slice::index::slice_end_index_len_fail 
not:33                                                    !~~~  error: no match expected
      116:  tail call void @_ZN4core5slice5index24slice_end_index_len_fail17h21e90144e0668f52E(i64 noundef %_19.0.i, i64 noundef 0, %"core::panic::location::Location<'_>"* noalias noundef nonnull readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc25 to %"core::panic::location::Location<'_>"*)) #4, !noalias !33 
not:32                                                                                                                                      !~~~~                                                                                                                                                                                               error: no match expected
      117:  unreachable 
      118:  
      119: "_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h811abdf3acb74a66E.exit.i": ; preds = %bb2.i 
      120:  %_14.0.i = sub i64 %_20.1.i, %_3.i 
      121:  %2 = getelementptr inbounds %"core::slice::iter::Windows<'_, u32>", %"core::slice::iter::Windows<'_, u32>"* %w, i64 0, i32 0, i32 0 
      122:  %self.0.i = load [0 x i32]*, [0 x i32]** %2, align 8, !alias.scope !30, !nonnull !25, !align !26 
      123:  %3 = getelementptr inbounds [0 x i32], [0 x i32]* %self.0.i, i64 0, i64 %_14.0.i 
      124:  store i64 %_19.0.i, i64* %1, align 8, !alias.scope !30 
      125:  br label %"_ZN109_$LT$core..slice..iter..Windows$LT$T$GT$$u20$as$u20$core..iter..traits..double_ended..DoubleEndedIterator$GT$9next_back17hb0e8283b5a9894bbE.exit" 
      126:  
      127: "_ZN109_$LT$core..slice..iter..Windows$LT$T$GT$$u20$as$u20$core..iter..traits..double_ended..DoubleEndedIterator$GT$9next_back17hb0e8283b5a9894bbE.exit": ; preds = %start, %"_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h811abdf3acb74a66E.exit.i" 
      128:  %.sroa.0.0.i = phi i32* [ %3, %"_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h811abdf3acb74a66E.exit.i" ], [ null, %start ] 
      129:  %4 = insertvalue { i32*, i64 } undef, i32* %.sroa.0.0.i, 0 
      130:  %5 = insertvalue { i32*, i64 } %4, i64 %_3.i, 1 
      131:  ret { i32*, i64 } %5 
      132: } 
      134: ; core::slice::index::slice_end_index_len_fail 
      134: ; core::slice::index::slice_end_index_len_fail 
      135: ; Function Attrs: cold noinline noreturn nonlazybind uwtable 
      136: declare void @_ZN4core5slice5index24slice_end_index_len_fail17h21e90144e0668f52E(i64 noundef, i64 noundef, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #1 
      138: ; core::slice::index::slice_start_index_len_fail 
      138: ; core::slice::index::slice_start_index_len_fail 
      139: ; Function Attrs: cold noinline noreturn nonlazybind uwtable 
      140: declare void @_ZN4core5slice5index26slice_start_index_len_fail17h97aba30d51dc1f1eE(i64 noundef, i64 noundef, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #1 
      141:  
      142: ; Function Attrs: nonlazybind uwtable 
      143: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, %"unwind::libunwind::_Unwind_Exception"* noundef, %"unwind::libunwind::_Unwind_Context"* noundef) unnamed_addr #0 
      144:  
      145: ; Function Attrs: argmemonly nofree nounwind nonlazybind readonly willreturn 
      146: declare i32 @bcmp(i8* nocapture, i8* nocapture, i64) local_unnamed_addr #2 
      147:  
      148: ; Function Attrs: inaccessiblememonly nofree nosync nounwind willreturn 
      149: declare void @llvm.experimental.noalias.scope.decl(metadata) #3 
      150:  
      151: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
      152: attributes #1 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
      153: attributes #2 = { argmemonly nofree nounwind nonlazybind readonly willreturn } 
      154: attributes #3 = { inaccessiblememonly nofree nosync nounwind willreturn } 
      155: attributes #4 = { noreturn } 
      156: attributes #5 = { nounwind } 
      157:  
      158: !llvm.module.flags = !{!0, !1} 
      159:  
      160: !0 = !{i32 7, !"PIC Level", i32 2} 
      161: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
      162: !2 = !{!3, !5, !7, !9} 
      163: !3 = distinct !{!3, !4, !"_ZN110_$LT$core..ops..range..RangeFrom$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17hb465068cae665d4fE: %slice.0"} 
      164: !4 = distinct !{!4, !"_ZN110_$LT$core..ops..range..RangeFrom$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17hb465068cae665d4fE"} 
      165: !5 = distinct !{!5, !6, !"_ZN94_$LT$core..slice..iter..Windows$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hcf35dea2dca633dbE: %self"} 
      166: !6 = distinct !{!6, !"_ZN94_$LT$core..slice..iter..Windows$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hcf35dea2dca633dbE"} 
      167: !7 = distinct !{!7, !8, !"_ZN4core4iter6traits8iterator8Iterator8try_fold17h81a78b758ebc1dbbE: %self"} 
      168: !8 = distinct !{!8, !"_ZN4core4iter6traits8iterator8Iterator8try_fold17h81a78b758ebc1dbbE"} 
      169: !9 = distinct !{!9, !8, !"_ZN4core4iter6traits8iterator8Iterator8try_fold17h81a78b758ebc1dbbE: argument 1"} 
      170: !10 = !{!11, !13, !14, !16} 
      171: !11 = distinct !{!11, !12, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17hd7221fa790ebea15E: %self.0"} 
      172: !12 = distinct !{!12, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17hd7221fa790ebea15E"} 
      173: !13 = distinct !{!13, !12, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17hd7221fa790ebea15E: %other.0"} 
      174: !14 = distinct !{!14, !15, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h0f526322d764c5e3E: %self.0"} 
      175: !15 = distinct !{!15, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h0f526322d764c5e3E"} 
      176: !16 = distinct !{!16, !15, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h0f526322d764c5e3E: %other.0"} 
      177: !17 = !{!18, !20, !7, !9} 
      178: !18 = distinct !{!18, !19, !"_ZN29slice_windows_no_bounds_check19naive_string_search28_$u7b$$u7b$closure$u7d$$u7d$17h7972d1e00e5230e9E: %_1"} 
      179: !19 = distinct !{!19, !"_ZN29slice_windows_no_bounds_check19naive_string_search28_$u7b$$u7b$closure$u7d$$u7d$17h7972d1e00e5230e9E"} 
      180: !20 = distinct !{!20, !21, !"_ZN4core4iter6traits8iterator8Iterator8position5check28_$u7b$$u7b$closure$u7d$$u7d$17h22cf92db040beb04E: %_1"} 
      181: !21 = distinct !{!21, !"_ZN4core4iter6traits8iterator8Iterator8position5check28_$u7b$$u7b$closure$u7d$$u7d$17h22cf92db040beb04E"} 
      182: !22 = !{!23} 
      183: !23 = distinct !{!23, !24, !"_ZN94_$LT$core..slice..iter..Windows$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0caf6eb78e720133E: %self"} 
      184: !24 = distinct !{!24, !"_ZN94_$LT$core..slice..iter..Windows$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h0caf6eb78e720133E"} 
      185: !25 = !{} 
      186: !26 = !{i64 4} 
      187: !27 = !{!28, !23} 
      188: !28 = distinct !{!28, !29, !"_ZN110_$LT$core..ops..range..RangeFrom$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17ha866c018f633b262E: %slice.0"} 
      189: !29 = distinct !{!29, !"_ZN110_$LT$core..ops..range..RangeFrom$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17ha866c018f633b262E"} 
      190: !30 = !{!31} 
      191: !31 = distinct !{!31, !32, !"_ZN109_$LT$core..slice..iter..Windows$LT$T$GT$$u20$as$u20$core..iter..traits..double_ended..DoubleEndedIterator$GT$9next_back17hb0e8283b5a9894bbE: %self"} 
      192: !32 = distinct !{!32, !"_ZN109_$LT$core..slice..iter..Windows$LT$T$GT$$u20$as$u20$core..iter..traits..double_ended..DoubleEndedIterator$GT$9next_back17hb0e8283b5a9894bbE"} 
      193: !33 = !{!34, !31} 
      194: !34 = distinct !{!34, !35, !"_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h811abdf3acb74a66E: %slice.0"} 
      195: !35 = distinct !{!35, !"_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h811abdf3acb74a66E"} 
------------------------------------------



