plain
failures:

---- [codegen] tests/codegen/align-offset.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-15/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-offset/align-offset.ll" "/checkout/tests/codegen/align-offset.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/align-offset.rs:19:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/align-offset.rs:19:12: error: CHECK: expected string not found in input
 // CHECK: ret i1 true
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-offset/align-offset.ll:21:37: note: scanning from here
define noundef zeroext i1 @align_to4(ptr noalias noundef nonnull readonly align 1 %x.0, i64 noundef %x.1) unnamed_addr #1 personality ptr @rust_eh_personality {
                                    ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-offset/align-offset.ll:57:2: note: possible intended match here
 ret i1 %_6


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-offset/align-offset.ll
Check file: /checkout/tests/codegen/align-offset.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'align_offset.d4a577fa05bd2dac-cgu.0' 
            2: source_filename = "align_offset.d4a577fa05bd2dac-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: @alloc_11195730e5236cfdc15ea13be1c301f9 = private unnamed_addr constant <{ [162 x i8] }> <{ [162 x i8] c"unsafe precondition(s) violated: slice::from_raw_parts requires the pointer to be aligned and non-null, and the total size of the slice not to exceed `isize::MAX`" }>, align 1 
            7:  
            8: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn uwtable 
            9: define noundef zeroext i1 @align8(ptr noundef %p) unnamed_addr #0 { 
           10: start: 
           11:  %addr.i.i = ptrtoint ptr %p to i64 
           12:  %_11.i.i = add i64 %addr.i.i, 7 
           13:  %aligned_address.i.i = and i64 %_11.i.i, -8 
           14:  %byte_offset.i.i = sub i64 %aligned_address.i.i, %addr.i.i 
           15:  %_15.i.i = icmp ult i64 %byte_offset.i.i, 8 
           16:  tail call void @llvm.assume(i1 %_15.i.i) 
           17:  ret i1 true 
           18: } 
           19:  
           20: ; Function Attrs: nounwind nonlazybind uwtable 
           21: define noundef zeroext i1 @align_to4(ptr noalias noundef nonnull readonly align 1 %x.0, i64 noundef %x.1) unnamed_addr #1 personality ptr @rust_eh_personality { 
check:19'0                                         X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           22: start: 
check:19'0     ~~~~~~~
           23:  %addr.i.i = ptrtoint ptr %x.0 to i64 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24:  %_11.i.i = add i64 %addr.i.i, 3 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25:  %aligned_address.i.i = and i64 %_11.i.i, -4 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           26:  %byte_offset.i.i = sub i64 %aligned_address.i.i, %addr.i.i 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27:  %_15.i.i = icmp ult i64 %byte_offset.i.i, 4 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28:  tail call void @llvm.assume(i1 %_15.i.i) 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           29:  %_8.i = icmp ugt i64 %byte_offset.i.i, %x.1 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           30:  br i1 %_8.i, label %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8align_to17hcb258310f3a2c0b8E.exit", label %_ZN4core5slice3raw14from_raw_parts17h0df3ca8f2b6fae4aE.exit.i.i 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31:  
check:19'0     ~
           32: _ZN4core5slice3raw14from_raw_parts17h0df3ca8f2b6fae4aE.exit.i.i: ; preds = %start 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           33:  %_12.0.i.i = sub i64 %x.1, %byte_offset.i.i 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           34:  %_10.i2.i.i = icmp slt i64 %_12.0.i.i, 0 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           35:  br i1 %_10.i2.i.i, label %bb5.i3.i.i, label %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$18split_at_unchecked17h2ddef8ad37fa15b7E.exit.i" 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           36:  
check:19'0     ~
           37: bb5.i3.i.i: ; preds = %_ZN4core5slice3raw14from_raw_parts17h0df3ca8f2b6fae4aE.exit.i.i 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           38: ; call core::panicking::panic_nounwind 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39:  tail call void @_ZN4core9panicking14panic_nounwind17h786e28ca17ed9c2fE(ptr noalias noundef nonnull readonly align 1 @alloc_11195730e5236cfdc15ea13be1c301f9, i64 noundef 162) #5, !noalias !2 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  unreachable 
check:19'0     ~~~~~~~~~~~~~
           41:  
check:19'0     ~
           42: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$18split_at_unchecked17h2ddef8ad37fa15b7E.exit.i": ; preds = %_ZN4core5slice3raw14from_raw_parts17h0df3ca8f2b6fae4aE.exit.i.i 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           43:  %_10.i.i = getelementptr inbounds i8, ptr %x.0, i64 %byte_offset.i.i 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  %_26.i.i.i.i = ptrtoint ptr %_10.i.i to i64 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           45:  %_25.i.i.i.i = and i64 %_26.i.i.i.i, 3 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46:  %.not.i = icmp eq i64 %_25.i.i.i.i, 0 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47:  br i1 %.not.i, label %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8align_to17hcb258310f3a2c0b8E.exit", label %bb5.i3.i 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           48:  
check:19'0     ~
           49: bb5.i3.i: ; preds = %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$18split_at_unchecked17h2ddef8ad37fa15b7E.exit.i" 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50: ; call core::panicking::panic_nounwind 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           51:  tail call void @_ZN4core9panicking14panic_nounwind17h786e28ca17ed9c2fE(ptr noalias noundef nonnull readonly align 1 @alloc_11195730e5236cfdc15ea13be1c301f9, i64 noundef 162) #5, !noalias !8 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           52:  unreachable 
check:19'0     ~~~~~~~~~~~~~
           53:  
check:19'0     ~
           54: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8align_to17hcb258310f3a2c0b8E.exit": ; preds = %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$18split_at_unchecked17h2ddef8ad37fa15b7E.exit.i", %start 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55:  %self.1.sink.i = phi i64 [ %x.1, %start ], [ %byte_offset.i.i, %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$18split_at_unchecked17h2ddef8ad37fa15b7E.exit.i" ] 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           56:  %_6 = icmp ult i64 %self.1.sink.i, 4 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           57:  ret i1 %_6 
check:19'0     ~~~~~~~~~~~~
check:19'1      ?           possible intended match
           58: } 
check:19'0     ~~
           59:  
check:19'0     ~
           60: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn uwtable 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           61: define noundef i64 @align_offset_byte_ptr(ptr noundef %ptr) unnamed_addr #0 { 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           62: start: 
           63:  %addr.i.i = ptrtoint ptr %ptr to i64 
           64:  %_11.i.i = add i64 %addr.i.i, 31 
           65:  %aligned_address.i.i = and i64 %_11.i.i, -32 
           66:  %byte_offset.i.i = sub i64 %aligned_address.i.i, %addr.i.i 
           67:  %_15.i.i = icmp ult i64 %byte_offset.i.i, 32 
           68:  tail call void @llvm.assume(i1 %_15.i.i) 
           69:  ret i64 %byte_offset.i.i 
           70: } 
           71:  
           72: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn uwtable 
           73: define noundef i64 @align_offset_word_slice(ptr noalias noundef nonnull readonly align 4 %slice.0, i64 noundef %slice.1) unnamed_addr #0 { 
           74: start: 
           75:  %addr.i.i = ptrtoint ptr %slice.0 to i64 
           76:  %_11.i.i = add i64 %addr.i.i, 31 
           77:  %aligned_address.i.i = and i64 %_11.i.i, -32 
           78:  %byte_offset.i.i = sub i64 %aligned_address.i.i, %addr.i.i 
           79:  %_15.i.i = icmp ult i64 %byte_offset.i.i, 32 
           80:  tail call void @llvm.assume(i1 %_15.i.i) 
           81:  %0 = lshr exact i64 %byte_offset.i.i, 2 
           82:  ret i64 %0 
           83: } 
           84:  
           85: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn uwtable 
           86: define noundef i64 @align_offset_word_ptr(ptr noundef %ptr) unnamed_addr #0 { 
           87: start: 
           88:  %addr.i.i = ptrtoint ptr %ptr to i64 
           89:  %_11.i.i = add i64 %addr.i.i, 31 
           90:  %aligned_address.i.i = and i64 %_11.i.i, -32 
           91:  %byte_offset.i.i = sub i64 %aligned_address.i.i, %addr.i.i 
           92:  %_15.i.i = icmp ult i64 %byte_offset.i.i, 32 
           93:  tail call void @llvm.assume(i1 %_15.i.i) 
           94:  %0 = and i64 %addr.i.i, 3 
           95:  %1 = icmp eq i64 %0, 0 
           96:  %2 = lshr exact i64 %byte_offset.i.i, 2 
           97:  %.0.i.i = select i1 %1, i64 %2, i64 -1 
           98:  ret i64 %.0.i.i 
           99: } 
          100:  
          101: ; Function Attrs: nonlazybind uwtable 
          102: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #2 
          104: ; core::panicking::panic_nounwind 
          104: ; core::panicking::panic_nounwind 
          105: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
          106: declare void @_ZN4core9panicking14panic_nounwind17h786e28ca17ed9c2fE(ptr noalias noundef nonnull readonly align 1, i64 noundef) unnamed_addr #3 
          107:  
          108: ; Function Attrs: inaccessiblememonly mustprogress nocallback nofree nosync nounwind willreturn 
          109: declare void @llvm.assume(i1 noundef) #4 
          110:  
          111: attributes #0 = { mustprogress nofree nosync nounwind nonlazybind willreturn uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
          112: attributes #1 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
          113: attributes #2 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
          114: attributes #3 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
          115: attributes #4 = { inaccessiblememonly mustprogress nocallback nofree nosync nounwind willreturn } 
          116: attributes #5 = { noreturn nounwind } 
          117:  
          118: !llvm.module.flags = !{!0, !1} 
          119:  
          120: !0 = !{i32 7, !"PIC Level", i32 2} 
          121: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
          122: !2 = !{!3, !5, !6} 
          123: !3 = distinct !{!3, !4, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$18split_at_unchecked17h2ddef8ad37fa15b7E: argument 0"} 
          124: !4 = distinct !{!4, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$18split_at_unchecked17h2ddef8ad37fa15b7E"} 
          125: !5 = distinct !{!5, !4, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$18split_at_unchecked17h2ddef8ad37fa15b7E: %self.0"} 
          126: !6 = distinct !{!6, !7, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8align_to17hcb258310f3a2c0b8E: argument 0"} 
          127: !7 = distinct !{!7, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8align_to17hcb258310f3a2c0b8E"} 
          128: !8 = !{!6} 
------------------------------------------



