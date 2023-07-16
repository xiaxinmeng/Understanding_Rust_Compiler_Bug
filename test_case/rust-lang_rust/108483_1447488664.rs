plain
failures:

---- [codegen] tests/codegen/slice-ref-equality.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll" "/checkout/tests/codegen/slice-ref-equality.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/slice-ref-equality.rs:53:12: error: CHECK: expected string not found in input
 // CHECK: tail call noundef i32 @{{bcmp|memcmp}}({{i8\*|ptr}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll:39:24: note: scanning from here
 %4 = mul nsw i32 %1, 3
                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll:40:6: note: possible intended match here
 %bcmp.i.i = tail call i32 @bcmp(ptr nonnull %0, ptr nonnull %2, i32 %4), !alias.scope !8
/checkout/tests/codegen/slice-ref-equality.rs:65:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/slice-ref-equality.rs:65:12: error: CHECK: expected string not found in input
 // CHECK: tail call noundef i32 @{{bcmp|memcmp}}({{i32\*|ptr}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll:56:24: note: scanning from here
 %4 = shl nsw i32 %1, 2
                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll:57:5: note: possible intended match here
 %bcmp.i.i = tail call i32 @bcmp(ptr nonnull %0, ptr nonnull %2, i32 %4), !alias.scope !15
/checkout/tests/codegen/slice-ref-equality.rs:77:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/slice-ref-equality.rs:77:12: error: CHECK: expected string not found in input
 // CHECK: tail call noundef i32 @{{bcmp|memcmp}}({{i32\*|ptr}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll:73:24: note: scanning from here
 %4 = shl nsw i32 %1, 2
                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll:74:5: note: possible intended match here
 %bcmp.i.i = tail call i32 @bcmp(ptr nonnull %0, ptr nonnull %2, i32 %4), !alias.scope !22
/checkout/tests/codegen/slice-ref-equality.rs:89:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/slice-ref-equality.rs:89:12: error: CHECK: expected string not found in input
 // CHECK: tail call noundef i32 @{{bcmp|memcmp}}({{i16\*|ptr}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll:90:24: note: scanning from here
 %4 = shl nsw i32 %1, 1
                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll:91:5: note: possible intended match here
 %bcmp.i.i = tail call i32 @bcmp(ptr nonnull %0, ptr nonnull %2, i32 %4), !alias.scope !29

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll
Check file: /checkout/tests/codegen/slice-ref-equality.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'slice_ref_equality.a912cc19-cgu.0' 
            2: source_filename = "slice_ref_equality.a912cc19-cgu.0" 
            3: target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128" 
            4: target triple = "i586-unknown-linux-gnu" 
            5:  
            6: @alloc3 = private unnamed_addr constant <{ [456 x i8] }> zeroinitializer, align 1 
            7:  
            8: ; Function Attrs: mustprogress nofree nounwind nonlazybind willreturn uwtable 
            9: define noundef zeroext i1 @is_zero_slice_long(ptr noalias noundef readonly align 1 dereferenceable(456) %data) unnamed_addr #0 { 
           10: bb6: 
           11:  %bcmp.i = tail call i32 @bcmp(ptr noundef nonnull dereferenceable(456) %data, ptr noundef nonnull dereferenceable(456) @alloc3, i32 456) 
           12:  %0 = icmp eq i32 %bcmp.i, 0 
           13:  ret i1 %0 
           14: } 
           15:  
           16: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn uwtable 
           17: define noundef zeroext i1 @is_zero_slice_short(ptr noalias noundef readonly align 1 dereferenceable(4) %data) unnamed_addr #1 { 
           18: bb6: 
           19:  %0 = load i32, ptr %data, align 1, !alias.scope !2 
           20:  %1 = icmp eq i32 %0, 0 
           21:  ret i1 %1 
           22: } 
           23:  
           24: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn uwtable 
           25: define noundef zeroext i1 @is_zero_array(ptr noalias nocapture noundef readonly align 1 dereferenceable(4) %data) unnamed_addr #1 { 
           26: start: 
           27:  %0 = load i32, ptr %data, align 1, !alias.scope !5 
           28:  %1 = icmp eq i32 %0, 0 
           29:  ret i1 %1 
           30: } 
           31:  
           32: ; Function Attrs: mustprogress nofree nounwind nonlazybind willreturn uwtable 
           33: define noundef zeroext i1 @eq_slice_of_nested_u8(ptr noalias nocapture noundef nonnull readonly align 1 %0, i32 noundef %1, ptr noalias nocapture noundef nonnull readonly align 1 %2, i32 noundef %3) unnamed_addr #0 { 
           34: start: 
           35:  %_3.not.i.i = icmp eq i32 %1, %3 
           36:  br i1 %_3.not.i.i, label %bb2.i.i, label %"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17hbb5bba222d4025fbE.exit" 
           37:  
           38: bb2.i.i: ; preds = %start 
           39:  %4 = mul nsw i32 %1, 3 
check:53'0                            X error: no match found
           40:  %bcmp.i.i = tail call i32 @bcmp(ptr nonnull %0, ptr nonnull %2, i32 %4), !alias.scope !8 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:53'1          ?                                                                                     possible intended match
           41:  %5 = icmp eq i32 %bcmp.i.i, 0 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42:  br label %"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17hbb5bba222d4025fbE.exit" 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           43:  
check:53'0     ~
           44: "_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17hbb5bba222d4025fbE.exit": ; preds = %start, %bb2.i.i 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           45:  %.0.i.i = phi i1 [ %5, %bb2.i.i ], [ false, %start ] 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46:  ret i1 %.0.i.i 
check:53'0     ~~~~~~~~~~~~~~~~
           47: } 
check:53'0     ~~
           48:  
check:53'0     ~
           49: ; Function Attrs: mustprogress nofree nounwind nonlazybind willreturn uwtable 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50: define noundef zeroext i1 @eq_slice_of_i32(ptr noalias nocapture noundef nonnull readonly align 4 %0, i32 noundef %1, ptr noalias nocapture noundef nonnull readonly align 4 %2, i32 noundef %3) unnamed_addr #0 { 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           51: start: 
           52:  %_3.not.i.i = icmp eq i32 %1, %3 
           53:  br i1 %_3.not.i.i, label %bb2.i.i, label %"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h7f8ef05cee5c41baE.exit" 
           54:  
           55: bb2.i.i: ; preds = %start 
           56:  %4 = shl nsw i32 %1, 2 
check:65'0                            X error: no match found
           57:  %bcmp.i.i = tail call i32 @bcmp(ptr nonnull %0, ptr nonnull %2, i32 %4), !alias.scope !15 
check:65'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:65'1         ?                                                                                       possible intended match
           58:  %5 = icmp eq i32 %bcmp.i.i, 0 
check:65'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           59:  br label %"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h7f8ef05cee5c41baE.exit" 
check:65'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           60:  
check:65'0     ~
           61: "_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h7f8ef05cee5c41baE.exit": ; preds = %start, %bb2.i.i 
check:65'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           62:  %.0.i.i = phi i1 [ %5, %bb2.i.i ], [ false, %start ] 
check:65'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63:  ret i1 %.0.i.i 
check:65'0     ~~~~~~~~~~~~~~~~
           64: } 
check:65'0     ~~
           65:  
check:65'0     ~
           66: ; Function Attrs: mustprogress nofree nounwind nonlazybind willreturn uwtable 
check:65'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           67: define noundef zeroext i1 @eq_slice_of_nonzero(ptr noalias nocapture noundef nonnull readonly align 4 %0, i32 noundef %1, ptr noalias nocapture noundef nonnull readonly align 4 %2, i32 noundef %3) unnamed_addr #0 { 
check:65'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           68: start: 
           69:  %_3.not.i.i = icmp eq i32 %1, %3 
           70:  br i1 %_3.not.i.i, label %bb2.i.i, label %"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h63773da68ea669daE.exit" 
           71:  
           72: bb2.i.i: ; preds = %start 
           73:  %4 = shl nsw i32 %1, 2 
check:77'0                            X error: no match found
           74:  %bcmp.i.i = tail call i32 @bcmp(ptr nonnull %0, ptr nonnull %2, i32 %4), !alias.scope !22 
check:77'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:77'1         ?                                                                                       possible intended match
           75:  %5 = icmp eq i32 %bcmp.i.i, 0 
check:77'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           76:  br label %"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h63773da68ea669daE.exit" 
check:77'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           77:  
check:77'0     ~
           78: "_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h63773da68ea669daE.exit": ; preds = %start, %bb2.i.i 
check:77'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           79:  %.0.i.i = phi i1 [ %5, %bb2.i.i ], [ false, %start ] 
check:77'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           80:  ret i1 %.0.i.i 
check:77'0     ~~~~~~~~~~~~~~~~
           81: } 
check:77'0     ~~
           82:  
check:77'0     ~
           83: ; Function Attrs: mustprogress nofree nounwind nonlazybind willreturn uwtable 
check:77'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           84: define noundef zeroext i1 @eq_slice_of_option_of_nonzero(ptr noalias nocapture noundef nonnull readonly align 2 %0, i32 noundef %1, ptr noalias nocapture noundef nonnull readonly align 2 %2, i32 noundef %3) unnamed_addr #0 { 
check:77'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           85: start: 
           86:  %_3.not.i.i = icmp eq i32 %1, %3 
           87:  br i1 %_3.not.i.i, label %bb2.i.i, label %"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17ha36bfa22ff2be486E.exit" 
           88:  
           89: bb2.i.i: ; preds = %start 
           90:  %4 = shl nsw i32 %1, 1 
check:89'0                            X error: no match found
           91:  %bcmp.i.i = tail call i32 @bcmp(ptr nonnull %0, ptr nonnull %2, i32 %4), !alias.scope !29 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:89'1         ?                                                                                       possible intended match
           92:  %5 = icmp eq i32 %bcmp.i.i, 0 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           93:  br label %"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17ha36bfa22ff2be486E.exit" 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           94:  
check:89'0     ~
           95: "_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17ha36bfa22ff2be486E.exit": ; preds = %start, %bb2.i.i 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           96:  %.0.i.i = phi i1 [ %5, %bb2.i.i ], [ false, %start ] 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           97:  ret i1 %.0.i.i 
check:89'0     ~~~~~~~~~~~~~~~~
           98: } 
check:89'0     ~~
           99:  
check:89'0     ~
          100: ; Function Attrs: argmemonly nofree nounwind nonlazybind readonly willreturn 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          101: declare i32 @bcmp(ptr nocapture, ptr nocapture, i32) local_unnamed_addr #2 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          102:  
check:89'0     ~
          103: attributes #0 = { mustprogress nofree nounwind nonlazybind willreturn uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          104: attributes #1 = { mustprogress nofree nosync nounwind nonlazybind willreturn uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          105: attributes #2 = { argmemonly nofree nounwind nonlazybind readonly willreturn } 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          106:  
check:89'0     ~
          107: !llvm.module.flags = !{!0, !1} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          108:  
check:89'0     ~
          109: !0 = !{i32 7, !"PIC Level", i32 2} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          110: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          111: !2 = !{!3} 
check:89'0     ~~~~~~~~~~~
          112: !3 = distinct !{!3, !4, !"_ZN69_$LT$T$u20$as$u20$core..array..equality..SpecArrayEq$LT$U$C$_$GT$$GT$7spec_eq17h7e97d28dd963e439E: %a"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          113: !4 = distinct !{!4, !"_ZN69_$LT$T$u20$as$u20$core..array..equality..SpecArrayEq$LT$U$C$_$GT$$GT$7spec_eq17h7e97d28dd963e439E"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          114: !5 = !{!6} 
check:89'0     ~~~~~~~~~~~
          115: !6 = distinct !{!6, !7, !"_ZN69_$LT$T$u20$as$u20$core..array..equality..SpecArrayEq$LT$U$C$_$GT$$GT$7spec_eq17h7e97d28dd963e439E: %a"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          116: !7 = distinct !{!7, !"_ZN69_$LT$T$u20$as$u20$core..array..equality..SpecArrayEq$LT$U$C$_$GT$$GT$7spec_eq17h7e97d28dd963e439E"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          117: !8 = !{!9, !11, !12, !14} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
          118: !9 = distinct !{!9, !10, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h58b153b9283709b5E: %self.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          119: !10 = distinct !{!10, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h58b153b9283709b5E"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          120: !11 = distinct !{!11, !10, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h58b153b9283709b5E: %other.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          121: !12 = distinct !{!12, !13, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17hbb5bba222d4025fbE: %self.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          122: !13 = distinct !{!13, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17hbb5bba222d4025fbE"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          123: !14 = distinct !{!14, !13, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17hbb5bba222d4025fbE: %other.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          124: !15 = !{!16, !18, !19, !21} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          125: !16 = distinct !{!16, !17, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h3cc7d1beede63e63E: %self.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          126: !17 = distinct !{!17, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h3cc7d1beede63e63E"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          127: !18 = distinct !{!18, !17, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h3cc7d1beede63e63E: %other.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          128: !19 = distinct !{!19, !20, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h7f8ef05cee5c41baE: %self.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          129: !20 = distinct !{!20, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h7f8ef05cee5c41baE"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          130: !21 = distinct !{!21, !20, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h7f8ef05cee5c41baE: %other.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          131: !22 = !{!23, !25, !26, !28} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          132: !23 = distinct !{!23, !24, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h8714f5a8cf30edbbE: %self.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          133: !24 = distinct !{!24, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h8714f5a8cf30edbbE"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          134: !25 = distinct !{!25, !24, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h8714f5a8cf30edbbE: %other.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          135: !26 = distinct !{!26, !27, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h63773da68ea669daE: %self.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          136: !27 = distinct !{!27, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h63773da68ea669daE"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          137: !28 = distinct !{!28, !27, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17h63773da68ea669daE: %other.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          138: !29 = !{!30, !32, !33, !35} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          139: !30 = distinct !{!30, !31, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h4948e9da36952723E: %self.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          140: !31 = distinct !{!31, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h4948e9da36952723E"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          141: !32 = distinct !{!32, !31, !"_ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h4948e9da36952723E: %other.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          142: !33 = distinct !{!33, !34, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17ha36bfa22ff2be486E: %self.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          143: !34 = distinct !{!34, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17ha36bfa22ff2be486E"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          144: !35 = distinct !{!35, !34, !"_ZN4core5slice3cmp81_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u5d$$GT$$u20$for$u20$$u5b$A$u5d$$GT$2eq17ha36bfa22ff2be486E: %other.0"} 
check:89'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



