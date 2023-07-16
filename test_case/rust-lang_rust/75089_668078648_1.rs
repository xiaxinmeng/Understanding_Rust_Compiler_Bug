
; ModuleID = 'rs_nostd_core_lib_1.bx7z8pw2-cgu.0'
source_filename = "rs_nostd_core_lib_1.bx7z8pw2-cgu.0"
target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
target triple = "wasm32-unknown-emscripten"

%Wrapper = type { [0 x i32], { [0 x i8]*, i32 }, [0 x i32], i32, [0 x i32] }
%"core::fmt::Arguments" = type { [0 x i32], { [0 x { [0 x i8]*, i32 }]*, i32 }, [0 x i32], { i32*, i32 }, [0 x i32], { [0 x { i8*, i32* }]*, i32 }, [0 x i32] }
%"core::fmt::Error" = type {}
%"core::fmt::Formatter" = type { [0 x i32], i32, [0 x i32], i32, [0 x i32], { i32, i32 }, [0 x i32], { i32, i32 }, [0 x i32], { {}*, [3 x i32]* }, [0 x i8], i8, [3 x i8] }
%"core::panic::Location" = type { [0 x i32], { [0 x i8]*, i32 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }
%"core::panic::PanicInfo" = type { [0 x i32], { {}*, [3 x i32]* }, [0 x i32], i32*, [0 x i32], %"core::panic::Location"*, [0 x i32] }

@vtable.0 = private unnamed_addr constant { void (%Wrapper**)*, i32, i32, i1 (%Wrapper**, [0 x i8]*, i32)*, i1 (%Wrapper**, i32)*, i1 (%Wrapper**, %"core::fmt::Arguments"*)* } { void (%Wrapper**)* bitcast (void (%"core::fmt::Error"*)* @_ZN4core3ptr13drop_in_place17h68e4b52a0cbffe94E to void (%Wrapper**)*), i32 4, i32 4, i1 (%Wrapper**, [0 x i8]*, i32)* @"_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17he860092712b03f97E", i1 (%Wrapper**, i32)* @"_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hf916e7144fe3b8a9E", i1 (%Wrapper**, %"core::fmt::Arguments"*)* @"_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hc58c3e48b4a70f96E" }, align 4
@alloc3 = private unnamed_addr constant <{ [0 x i8] }> zeroinitializer, align 4
@alloc43 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Result::unwrap()` on an `Err` value" }>, align 1
@vtable.1 = private unnamed_addr constant { void (%"core::fmt::Error"*)*, i32, i32, i1 (%"core::fmt::Error"*, %"core::fmt::Formatter"*)* } { void (%"core::fmt::Error"*)* @_ZN4core3ptr13drop_in_place17h68e4b52a0cbffe94E, i32 0, i32 1, i1 (%"core::fmt::Error"*, %"core::fmt::Formatter"*)* @"_ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h91893e32323f1213E" }, align 4
@alloc48 = private unnamed_addr constant <{ [10 x i8] }> <{ [10 x i8] c"src/lib.rs" }>, align 1
@alloc45 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [10 x i8] }>, <{ [10 x i8] }>* @alloc48, i32 0, i32 0, i32 0), [12 x i8] c"\0A\00\00\00!\00\00\00\1E\00\00\00" }>, align 4
@alloc5 = private unnamed_addr constant <{ [3 x i8] }> <{ [3 x i8] c" + " }>, align 1
@alloc6 = private unnamed_addr constant <{ [3 x i8] }> <{ [3 x i8] c" = " }>, align 1
@alloc4 = private unnamed_addr constant <{ i8*, [4 x i8], i8*, [4 x i8], i8*, [4 x i8] }> <{ i8* getelementptr inbounds (<{ [0 x i8] }>, <{ [0 x i8] }>* @alloc3, i32 0, i32 0, i32 0), [4 x i8] zeroinitializer, i8* getelementptr inbounds (<{ [3 x i8] }>, <{ [3 x i8] }>* @alloc5, i32 0, i32 0, i32 0), [4 x i8] c"\03\00\00\00", i8* getelementptr inbounds (<{ [3 x i8] }>, <{ [3 x i8] }>* @alloc6, i32 0, i32 0, i32 0), [4 x i8] c"\03\00\00\00" }>, align 4
@alloc49 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [10 x i8] }>, <{ [10 x i8] }>* @alloc48, i32 0, i32 0, i32 0), [12 x i8] c"\0A\00\00\005\00\00\00A\00\00\00" }>, align 4

; core::ptr::drop_in_place
; Function Attrs: norecurse nounwind readnone
define internal void @_ZN4core3ptr13drop_in_place17h68e4b52a0cbffe94E(%"core::fmt::Error"* nocapture %_1) unnamed_addr #0 {
start:
  ret void
}

; <&mut W as core::fmt::Write>::write_char
; Function Attrs: nounwind
define internal zeroext i1 @"_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hf916e7144fe3b8a9E"(%Wrapper** nocapture readonly align 4 dereferenceable(4) %self, i32 %c) unnamed_addr #1 {
start:
  %_10.i = alloca i32, align 4
  %_3 = load %Wrapper*, %Wrapper** %self, align 4, !nonnull !1
  %_10.i.0.sroa_cast5 = bitcast i32* %_10.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %_10.i.0.sroa_cast5)
  store i32 0, i32* %_10.i, align 4
  %0 = icmp ult i32 %c, 1114112
  tail call void @llvm.assume(i1 %0) #6
  %_2.i.i.i.i = icmp ult i32 %c, 128
  br i1 %_2.i.i.i.i, label %bb3.i.i.i, label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %start
  %_4.i.i.i.i = icmp ult i32 %c, 2048
  br i1 %_4.i.i.i.i, label %bb5.i.i.i, label %bb3.i.i.i.i

bb3.i.i.i.i:                                      ; preds = %bb1.i.i.i.i
  %_6.i.i.i.i = icmp ult i32 %c, 65536
  br i1 %_6.i.i.i.i, label %bb6.i.i.i, label %bb7.i.i.i

bb3.i.i.i:                                        ; preds = %start
  %1 = trunc i32 %c to i8
  store i8 %1, i8* %_10.i.0.sroa_cast5, align 4
  br label %"_ZN4core4char7methods22_$LT$impl$u20$char$GT$11encode_utf817h8b2344aa57c4604cE.exit.i"

bb5.i.i.i:                                        ; preds = %bb1.i.i.i.i
  %_30.i.i.i = lshr i32 %c, 6
  %2 = trunc i32 %_30.i.i.i to i8
  %3 = or i8 %2, -64
  store i8 %3, i8* %_10.i.0.sroa_cast5, align 4
  %4 = trunc i32 %c to i8
  %_32.i.i.i = and i8 %4, 63
  %5 = or i8 %_32.i.i.i, -128
  %_10.i.1.sroa_raw_idx10 = getelementptr inbounds i8, i8* %_10.i.0.sroa_cast5, i32 1
  store i8 %5, i8* %_10.i.1.sroa_raw_idx10, align 1
  br label %"_ZN4core4char7methods22_$LT$impl$u20$char$GT$11encode_utf817h8b2344aa57c4604cE.exit.i"

bb6.i.i.i:                                        ; preds = %bb3.i.i.i.i
  %_40.i.i.i = lshr i32 %c, 12
  %6 = trunc i32 %_40.i.i.i to i8
  %7 = or i8 %6, -32
  store i8 %7, i8* %_10.i.0.sroa_cast5, align 4
  %_44.i.i.i = lshr i32 %c, 6
  %8 = trunc i32 %_44.i.i.i to i8
  %_42.i.i.i = and i8 %8, 63
  %9 = or i8 %_42.i.i.i, -128
  %_10.i.1.sroa_raw_idx8 = getelementptr inbounds i8, i8* %_10.i.0.sroa_cast5, i32 1
  store i8 %9, i8* %_10.i.1.sroa_raw_idx8, align 1
  %10 = trunc i32 %c to i8
  %_46.i.i.i = and i8 %10, 63
  %11 = or i8 %_46.i.i.i, -128
  %_10.i.2.sroa_raw_idx12 = getelementptr inbounds i8, i8* %_10.i.0.sroa_cast5, i32 2
  store i8 %11, i8* %_10.i.2.sroa_raw_idx12, align 2
  br label %"_ZN4core4char7methods22_$LT$impl$u20$char$GT$11encode_utf817h8b2344aa57c4604cE.exit.i"

bb7.i.i.i:                                        ; preds = %bb3.i.i.i.i
  %_55.i.i.i = lshr i32 %c, 18
  %12 = trunc i32 %_55.i.i.i to i8
  %13 = or i8 %12, -16
  store i8 %13, i8* %_10.i.0.sroa_cast5, align 4
  %_59.i.i.i = lshr i32 %c, 12
  %14 = trunc i32 %_59.i.i.i to i8
  %_57.i.i.i = and i8 %14, 63
  %15 = or i8 %_57.i.i.i, -128
  %_10.i.1.sroa_raw_idx = getelementptr inbounds i8, i8* %_10.i.0.sroa_cast5, i32 1
  store i8 %15, i8* %_10.i.1.sroa_raw_idx, align 1
  %_63.i.i.i = lshr i32 %c, 6
  %16 = trunc i32 %_63.i.i.i to i8
  %_61.i.i.i = and i8 %16, 63
  %17 = or i8 %_61.i.i.i, -128
  %_10.i.2.sroa_raw_idx = getelementptr inbounds i8, i8* %_10.i.0.sroa_cast5, i32 2
  store i8 %17, i8* %_10.i.2.sroa_raw_idx, align 2
  %18 = trunc i32 %c to i8
  %_65.i.i.i = and i8 %18, 63
  %19 = or i8 %_65.i.i.i, -128
  %_10.i.3.sroa_raw_idx = getelementptr inbounds i8, i8* %_10.i.0.sroa_cast5, i32 3
  store i8 %19, i8* %_10.i.3.sroa_raw_idx, align 1
  br label %"_ZN4core4char7methods22_$LT$impl$u20$char$GT$11encode_utf817h8b2344aa57c4604cE.exit.i"

"_ZN4core4char7methods22_$LT$impl$u20$char$GT$11encode_utf817h8b2344aa57c4604cE.exit.i": ; preds = %bb7.i.i.i, %bb6.i.i.i, %bb5.i.i.i, %bb3.i.i.i
  %.0.i3.i.i.i = phi i32 [ 1, %bb3.i.i.i ], [ 2, %bb5.i.i.i ], [ 3, %bb6.i.i.i ], [ 4, %bb7.i.i.i ]
  %20 = getelementptr inbounds %Wrapper, %Wrapper* %_3, i32 0, i32 1, i32 1
  %_7.1.i.i = load i32, i32* %20, align 4, !noalias !2
  %21 = getelementptr inbounds %Wrapper, %Wrapper* %_3, i32 0, i32 3
  %_9.i.i = load i32, i32* %21, align 4, !noalias !2
  %_4.i.i.i1.i = icmp ult i32 %_7.1.i.i, %_9.i.i
  br i1 %_4.i.i.i1.i, label %bb3.i.i.i2.i, label %"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17haff892d5fa9f0e6fE.exit.i.i"

bb3.i.i.i2.i:                                     ; preds = %"_ZN4core4char7methods22_$LT$impl$u20$char$GT$11encode_utf817h8b2344aa57c4604cE.exit.i"
; call core::slice::slice_start_index_len_fail
  tail call void @_ZN4core5slice26slice_start_index_len_fail17h93a3e2aca99ea8b2E(i32 %_9.i.i, i32 %_7.1.i.i, %"core::panic::Location"* noalias readonly align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc45 to %"core::panic::Location"*)) #6, !noalias !2
  unreachable

"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17haff892d5fa9f0e6fE.exit.i.i": ; preds = %"_ZN4core4char7methods22_$LT$impl$u20$char$GT$11encode_utf817h8b2344aa57c4604cE.exit.i"
  %_7.i.i.i.i.i.i = sub i32 %_7.1.i.i, %_9.i.i
  %_10.i.i = icmp ult i32 %_7.i.i.i.i.i.i, %.0.i3.i.i.i
  br i1 %_10.i.i, label %_ZN4core3fmt5Write10write_char17h0d924c85b8f19216E.exit, label %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17ha21d5f43eb2f5f26E.exit.i.i"

"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17ha21d5f43eb2f5f26E.exit.i.i": ; preds = %"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17haff892d5fa9f0e6fE.exit.i.i"
  %22 = bitcast %Wrapper* %_3 to [0 x i8]**
  %_7.0.i.i = load [0 x i8]*, [0 x i8]** %22, align 4, !noalias !2, !nonnull !1
  %23 = getelementptr inbounds [0 x i8], [0 x i8]* %_7.0.i.i, i32 0, i32 %_9.i.i
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* nonnull align 1 %23, i8* nonnull align 4 %_10.i.0.sroa_cast5, i32 %.0.i3.i.i.i, i1 false) #6
  %24 = load i32, i32* %21, align 4, !noalias !2
  %25 = add i32 %24, %.0.i3.i.i.i
  store i32 %25, i32* %21, align 4, !noalias !2
  br label %_ZN4core3fmt5Write10write_char17h0d924c85b8f19216E.exit

_ZN4core3fmt5Write10write_char17h0d924c85b8f19216E.exit: ; preds = %"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17haff892d5fa9f0e6fE.exit.i.i", %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17ha21d5f43eb2f5f26E.exit.i.i"
  %.0.off0.i.i = phi i1 [ false, %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17ha21d5f43eb2f5f26E.exit.i.i" ], [ true, %"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17haff892d5fa9f0e6fE.exit.i.i" ]
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %_10.i.0.sroa_cast5)
  ret i1 %.0.off0.i.i
}

; <&mut W as core::fmt::Write>::write_fmt
; Function Attrs: nounwind
define internal zeroext i1 @"_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hc58c3e48b4a70f96E"(%Wrapper** nocapture readonly align 4 dereferenceable(4) %self, %"core::fmt::Arguments"* noalias nocapture readonly dereferenceable(24) %args) unnamed_addr #1 {
start:
  %_6.i = alloca %"core::fmt::Arguments", align 4
  %self.i = alloca %Wrapper*, align 4
  %0 = bitcast %Wrapper** %self to i32*
  %_32 = load i32, i32* %0, align 4, !range !5
  %1 = bitcast %"core::fmt::Arguments"* %args to i8*
  %2 = bitcast %Wrapper** %self.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %2)
  %3 = bitcast %Wrapper** %self.i to i32*
  store i32 %_32, i32* %3, align 4, !noalias !6
  %_3.0.i = bitcast %Wrapper** %self.i to {}*
  %4 = bitcast %"core::fmt::Arguments"* %_6.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %4) #6, !noalias !6
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* nonnull align 4 dereferenceable(24) %4, i8* nonnull align 4 dereferenceable(24) %1, i32 24, i1 false)
; call core::fmt::write
  %5 = call zeroext i1 @_ZN4core3fmt5write17ha38f87b9346b94a5E({}* nonnull align 1 %_3.0.i, [3 x i32]* noalias readonly align 4 dereferenceable(12) bitcast ({ void (%Wrapper**)*, i32, i32, i1 (%Wrapper**, [0 x i8]*, i32)*, i1 (%Wrapper**, i32)*, i1 (%Wrapper**, %"core::fmt::Arguments"*)* }* @vtable.0 to [3 x i32]*), %"core::fmt::Arguments"* noalias nocapture nonnull dereferenceable(24) %_6.i) #6, !noalias !6
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %4) #6, !noalias !6
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %2)
  ret i1 %5
}

; <&mut W as core::fmt::Write>::write_str
; Function Attrs: nounwind
define internal zeroext i1 @"_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17he860092712b03f97E"(%Wrapper** nocapture readonly align 4 dereferenceable(4) %self, [0 x i8]* noalias nocapture nonnull readonly align 1 %s.0, i32 %s.1) unnamed_addr #1 {
start:
  %_3 = load %Wrapper*, %Wrapper** %self, align 4, !nonnull !1
  %0 = getelementptr inbounds %Wrapper, %Wrapper* %_3, i32 0, i32 1, i32 1
  %_7.1.i = load i32, i32* %0, align 4, !noalias !9
  %1 = getelementptr inbounds %Wrapper, %Wrapper* %_3, i32 0, i32 3
  %_9.i = load i32, i32* %1, align 4, !noalias !9
  %_4.i.i.i = icmp ult i32 %_7.1.i, %_9.i
  br i1 %_4.i.i.i, label %bb3.i.i.i, label %"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17haff892d5fa9f0e6fE.exit.i"

bb3.i.i.i:                                        ; preds = %start
; call core::slice::slice_start_index_len_fail
  tail call void @_ZN4core5slice26slice_start_index_len_fail17h93a3e2aca99ea8b2E(i32 %_9.i, i32 %_7.1.i, %"core::panic::Location"* noalias readonly align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc45 to %"core::panic::Location"*)) #6, !noalias !9
  unreachable

"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17haff892d5fa9f0e6fE.exit.i": ; preds = %start
  %_7.i.i.i.i.i = sub i32 %_7.1.i, %_9.i
  %_10.i = icmp ult i32 %_7.i.i.i.i.i, %s.1
  br i1 %_10.i, label %"_ZN65_$LT$rs_nostd_core_lib_1..Wrapper$u20$as$u20$core..fmt..Write$GT$9write_str17h47eabec113995301E.exit", label %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17ha21d5f43eb2f5f26E.exit.i"

"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17ha21d5f43eb2f5f26E.exit.i": ; preds = %"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17haff892d5fa9f0e6fE.exit.i"
  %2 = bitcast %Wrapper* %_3 to [0 x i8]**
  %_7.0.i = load [0 x i8]*, [0 x i8]** %2, align 4, !noalias !9, !nonnull !1
  %3 = getelementptr inbounds [0 x i8], [0 x i8]* %_7.0.i, i32 0, i32 %_9.i
  %4 = getelementptr [0 x i8], [0 x i8]* %s.0, i32 0, i32 0
  tail call void @llvm.memcpy.p0i8.p0i8.i32(i8* nonnull align 1 %3, i8* nonnull align 1 %4, i32 %s.1, i1 false) #6
  %5 = load i32, i32* %1, align 4, !noalias !9
  %6 = add i32 %5, %s.1
  store i32 %6, i32* %1, align 4, !noalias !9
  br label %"_ZN65_$LT$rs_nostd_core_lib_1..Wrapper$u20$as$u20$core..fmt..Write$GT$9write_str17h47eabec113995301E.exit"

"_ZN65_$LT$rs_nostd_core_lib_1..Wrapper$u20$as$u20$core..fmt..Write$GT$9write_str17h47eabec113995301E.exit": ; preds = %"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17haff892d5fa9f0e6fE.exit.i", %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17ha21d5f43eb2f5f26E.exit.i"
  %.0.off0.i = phi i1 [ false, %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17ha21d5f43eb2f5f26E.exit.i" ], [ true, %"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17haff892d5fa9f0e6fE.exit.i" ]
  ret i1 %.0.off0.i
}

; Function Attrs: norecurse noreturn nounwind readnone
define hidden void @rust_begin_unwind(%"core::panic::PanicInfo"* noalias nocapture readonly align 4 dereferenceable(16) %_info) unnamed_addr #2 {
start:
  br label %bb1

bb1:                                              ; preds = %bb1, %start
  br label %bb1
}

; Function Attrs: nounwind
define i32 @rust_fn(i32 %0, i32 %1) unnamed_addr #1 {
start:
  %e.i = alloca %"core::fmt::Error", align 1
  %_6.i = alloca %"core::fmt::Arguments", align 4
  %self.i = alloca %Wrapper*, align 4
  %_23 = alloca i32, align 4
  %_18 = alloca [3 x { i8*, i32* }], align 4
  %_7 = alloca %Wrapper, align 4
  %buf = alloca [100 x i8], align 1
  %y = alloca i32, align 4
  %x = alloca i32, align 4
  store i32 %0, i32* %x, align 4
  store i32 %1, i32* %y, align 4
  %2 = getelementptr inbounds [100 x i8], [100 x i8]* %buf, i32 0, i32 0
  call void @llvm.lifetime.start.p0i8(i64 100, i8* nonnull %2)
  call void @llvm.memset.p0i8.i32(i8* nonnull align 1 dereferenceable(100) %2, i8 0, i32 100, i1 false)
  %3 = bitcast %Wrapper* %_7 to i8*
  call void @llvm.lifetime.start.p0i8(i64 12, i8* nonnull %3)
  %4 = bitcast %Wrapper* %_7 to [100 x i8]**
  store [100 x i8]* %buf, [100 x i8]** %4, align 4, !alias.scope !12
  %5 = getelementptr inbounds %Wrapper, %Wrapper* %_7, i32 0, i32 1, i32 1
  store i32 100, i32* %5, align 4, !alias.scope !12
  %6 = getelementptr inbounds %Wrapper, %Wrapper* %_7, i32 0, i32 3
  store i32 0, i32* %6, align 4, !alias.scope !12
  %7 = bitcast [3 x { i8*, i32* }]* %_18 to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %7)
  %8 = bitcast i32* %_23 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %8)
  %9 = add i32 %1, %0
  store i32 %9, i32* %_23, align 4
  %10 = bitcast [3 x { i8*, i32* }]* %_18 to i32**
  store i32* %x, i32** %10, align 4
  %11 = getelementptr inbounds [3 x { i8*, i32* }], [3 x { i8*, i32* }]* %_18, i32 0, i32 0, i32 1
  store i32* bitcast (i1 (i32*, %"core::fmt::Formatter"*)* @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hae3e67d76fb04481E" to i32*), i32** %11, align 4
  %12 = getelementptr inbounds [3 x { i8*, i32* }], [3 x { i8*, i32* }]* %_18, i32 0, i32 1, i32 0
  %13 = bitcast i8** %12 to i32**
  store i32* %y, i32** %13, align 4
  %14 = getelementptr inbounds [3 x { i8*, i32* }], [3 x { i8*, i32* }]* %_18, i32 0, i32 1, i32 1
  store i32* bitcast (i1 (i32*, %"core::fmt::Formatter"*)* @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hae3e67d76fb04481E" to i32*), i32** %14, align 4
  %15 = getelementptr inbounds [3 x { i8*, i32* }], [3 x { i8*, i32* }]* %_18, i32 0, i32 2, i32 0
  %16 = bitcast i8** %15 to i32**
  store i32* %_23, i32** %16, align 4
  %17 = getelementptr inbounds [3 x { i8*, i32* }], [3 x { i8*, i32* }]* %_18, i32 0, i32 2, i32 1
  store i32* bitcast (i1 (i32*, %"core::fmt::Formatter"*)* @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hae3e67d76fb04481E" to i32*), i32** %17, align 4
  %18 = bitcast %Wrapper** %self.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %18)
  store %Wrapper* %_7, %Wrapper** %self.i, align 4, !noalias !15
  %_3.0.i = bitcast %Wrapper** %self.i to {}*
  %19 = bitcast %"core::fmt::Arguments"* %_6.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %19) #6, !noalias !15
  %_11.sroa.0.0..sroa_cast = bitcast %"core::fmt::Arguments"* %_6.i to [0 x { [0 x i8]*, i32 }]**
  store [0 x { [0 x i8]*, i32 }]* bitcast (<{ i8*, [4 x i8], i8*, [4 x i8], i8*, [4 x i8] }>* @alloc4 to [0 x { [0 x i8]*, i32 }]*), [0 x { [0 x i8]*, i32 }]** %_11.sroa.0.0..sroa_cast, align 4
  %_11.sroa.4.0..sroa_idx8 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_6.i, i32 0, i32 1, i32 1
  store i32 3, i32* %_11.sroa.4.0..sroa_idx8, align 4
  %_11.sroa.5.0..sroa_idx10 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_6.i, i32 0, i32 3, i32 0
  store i32* null, i32** %_11.sroa.5.0..sroa_idx10, align 4
  %_11.sroa.613.0..sroa_idx15 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_6.i, i32 0, i32 5, i32 0
  %20 = bitcast [0 x { i8*, i32* }]** %_11.sroa.613.0..sroa_idx15 to [3 x { i8*, i32* }]**
  store [3 x { i8*, i32* }]* %_18, [3 x { i8*, i32* }]** %20, align 4
  %_11.sroa.7.0..sroa_idx17 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_6.i, i32 0, i32 5, i32 1
  store i32 3, i32* %_11.sroa.7.0..sroa_idx17, align 4
; call core::fmt::write
  %21 = call zeroext i1 @_ZN4core3fmt5write17ha38f87b9346b94a5E({}* nonnull align 1 %_3.0.i, [3 x i32]* noalias readonly align 4 dereferenceable(12) bitcast ({ void (%Wrapper**)*, i32, i32, i1 (%Wrapper**, [0 x i8]*, i32)*, i1 (%Wrapper**, i32)*, i1 (%Wrapper**, %"core::fmt::Arguments"*)* }* @vtable.0 to [3 x i32]*), %"core::fmt::Arguments"* noalias nocapture nonnull dereferenceable(24) %_6.i) #6, !noalias !15
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %19) #6, !noalias !15
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %18)
  %22 = bitcast %"core::fmt::Error"* %e.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 0, i8* nonnull %22)
  br i1 %21, label %bb1.i, label %"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h9818abcbf4c27260E.exit"

bb1.i:                                            ; preds = %start
  %_6.0.i = bitcast %"core::fmt::Error"* %e.i to {}*
; call core::result::unwrap_failed
  call void @_ZN4core6result13unwrap_failed17he13eee0d2f5c9131E([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [43 x i8] }>* @alloc43 to [0 x i8]*), i32 43, {}* nonnull align 1 %_6.0.i, [3 x i32]* noalias readonly align 4 dereferenceable(12) bitcast ({ void (%"core::fmt::Error"*)*, i32, i32, i1 (%"core::fmt::Error"*, %"core::fmt::Formatter"*)* }* @vtable.1 to [3 x i32]*), %"core::panic::Location"* noalias readonly align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc49 to %"core::panic::Location"*)) #6
  unreachable

"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h9818abcbf4c27260E.exit": ; preds = %start
  call void @llvm.lifetime.end.p0i8(i64 0, i8* nonnull %22)
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %8)
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %7)
  call void @llvm.lifetime.end.p0i8(i64 12, i8* nonnull %3)
  call void @llvm.lifetime.end.p0i8(i64 100, i8* nonnull %2)
  ret i32 0
}

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #3

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #3

; core::slice::slice_start_index_len_fail
; Function Attrs: cold noinline noreturn nounwind
declare void @_ZN4core5slice26slice_start_index_len_fail17h93a3e2aca99ea8b2E(i32, i32, %"core::panic::Location"* noalias readonly align 4 dereferenceable(16)) unnamed_addr #4

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #3

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memset.p0i8.i32(i8* nocapture writeonly, i8, i32, i1 immarg) #3

; core::fmt::write
; Function Attrs: nounwind
declare zeroext i1 @_ZN4core3fmt5write17ha38f87b9346b94a5E({}* nonnull align 1, [3 x i32]* noalias readonly align 4 dereferenceable(12), %"core::fmt::Arguments"* noalias nocapture dereferenceable(24)) unnamed_addr #1

; Function Attrs: nounwind willreturn
declare void @llvm.assume(i1) #5

; <core::fmt::Error as core::fmt::Debug>::fmt
; Function Attrs: nounwind
declare zeroext i1 @"_ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h91893e32323f1213E"(%"core::fmt::Error"* noalias nonnull readonly align 1, %"core::fmt::Formatter"* align 4 dereferenceable(36)) unnamed_addr #1

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nounwind
declare void @_ZN4core6result13unwrap_failed17he13eee0d2f5c9131E([0 x i8]* noalias nonnull readonly align 1, i32, {}* nonnull align 1, [3 x i32]* noalias readonly align 4 dereferenceable(12), %"core::panic::Location"* noalias readonly align 4 dereferenceable(16)) unnamed_addr #4

; core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
; Function Attrs: nounwind
declare zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hae3e67d76fb04481E"(i32* noalias readonly align 4 dereferenceable(4), %"core::fmt::Formatter"* align 4 dereferenceable(36)) unnamed_addr #1

attributes #0 = { norecurse nounwind readnone "target-cpu"="generic" }
attributes #1 = { nounwind "target-cpu"="generic" }
attributes #2 = { norecurse noreturn nounwind readnone "target-cpu"="generic" }
attributes #3 = { argmemonly nounwind willreturn }
attributes #4 = { cold noinline noreturn nounwind "target-cpu"="generic" }
attributes #5 = { nounwind willreturn }
attributes #6 = { nounwind }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_ZN65_$LT$rs_nostd_core_lib_1..Wrapper$u20$as$u20$core..fmt..Write$GT$9write_str17h47eabec113995301E: %s.0"}
!4 = distinct !{!4, !"_ZN65_$LT$rs_nostd_core_lib_1..Wrapper$u20$as$u20$core..fmt..Write$GT$9write_str17h47eabec113995301E"}
!5 = !{i32 1, i32 0}
!6 = !{!7}
!7 = distinct !{!7, !8, !"_ZN4core3fmt5Write9write_fmt17h370ccc85b178e033E: %args"}
!8 = distinct !{!8, !"_ZN4core3fmt5Write9write_fmt17h370ccc85b178e033E"}
!9 = !{!10}
!10 = distinct !{!10, !11, !"_ZN65_$LT$rs_nostd_core_lib_1..Wrapper$u20$as$u20$core..fmt..Write$GT$9write_str17h47eabec113995301E: %s.0"}
!11 = distinct !{!11, !"_ZN65_$LT$rs_nostd_core_lib_1..Wrapper$u20$as$u20$core..fmt..Write$GT$9write_str17h47eabec113995301E"}
!12 = !{!13}
!13 = distinct !{!13, !14, !"_ZN19rs_nostd_core_lib_17Wrapper3new17h4b2c92d71cd00f55E: argument 0"}
!14 = distinct !{!14, !"_ZN19rs_nostd_core_lib_17Wrapper3new17h4b2c92d71cd00f55E"}
!15 = !{!16}
!16 = distinct !{!16, !17, !"_ZN4core3fmt5Write9write_fmt17h370ccc85b178e033E: %args"}
!17 = distinct !{!17, !"_ZN4core3fmt5Write9write_fmt17h370ccc85b178e033E"}
