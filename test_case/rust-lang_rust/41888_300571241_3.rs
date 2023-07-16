llvm
; ModuleID = '1.cgu-0.rs'
source_filename = "1.cgu-0.rs"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-darwin"

%str_slice = type { i8*, i64 }
%"core::option::Option<(u32, u8)>" = type { i32, [0 x i32], [2 x i32] }
%X = type { i64, [0 x i64], [1 x i64] }
%"core::option::Option<E>" = type { i64, [0 x i64], [5 x i64] }
%E = type { i8, [7 x i8], [4 x i64] }
%"core::result::Result<(), (u32, u8)>" = type { i32, [0 x i32], [2 x i32] }
%"unwind::libunwind::_Unwind_Exception" = type { i64, [0 x i8], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i8], [6 x i64], [0 x i8] }
%"unwind::libunwind::_Unwind_Context" = type {}

@"_ZN38_$LT$core..option..Option$LT$T$GT$$GT$6unwrap14_MSG_FILE_LINE17he9e3266a381f7b99E" = external global { %str_slice, [0 x i8], %str_slice, [0 x i8], i32, [4 x i8] }

; Function Attrs: inlinehint
define internal fastcc i64 @"_ZN38_$LT$core..option..Option$LT$T$GT$$GT$6unwrap17hdbd5a86115aeeb48E"(%"core::option::Option<(u32, u8)>"* noalias nocapture readonly dereferenceable(12)) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %self.sroa.0.0..sroa_idx = getelementptr inbounds %"core::option::Option<(u32, u8)>", %"core::option::Option<(u32, u8)>"* %0, i64 0, i32 0
  %self.sroa.0.0.copyload = load i32, i32* %self.sroa.0.0..sroa_idx, align 4
  %cond = icmp eq i32 %self.sroa.0.0.copyload, 0
  br i1 %cond, label %bb3, label %bb4

bb3:                                              ; preds = %start
  tail call void @_ZN4core9panicking5panic17hafbe89720e5223c3E({ %str_slice, [0 x i8], %str_slice, [0 x i8], i32, [4 x i8] }* noalias nonnull readonly dereferenceable(40) @"_ZN38_$LT$core..option..Option$LT$T$GT$$GT$6unwrap14_MSG_FILE_LINE17he9e3266a381f7b99E")
  unreachable

bb4:                                              ; preds = %start
  %self.sroa.5.0..sroa_idx = getelementptr inbounds %"core::option::Option<(u32, u8)>", %"core::option::Option<(u32, u8)>"* %0, i64 0, i32 2, i64 1
  %self.sroa.5.0..sroa_cast = bitcast i32* %self.sroa.5.0..sroa_idx to i8*
  %self.sroa.5.0.copyload = load i8, i8* %self.sroa.5.0..sroa_cast, align 4
  %self.sroa.4.0..sroa_idx3 = getelementptr inbounds %"core::option::Option<(u32, u8)>", %"core::option::Option<(u32, u8)>"* %0, i64 0, i32 2, i64 0
  %self.sroa.4.0.copyload = load i32, i32* %self.sroa.4.0..sroa_idx3, align 4
  %ret.sroa.2.0.insert.ext = zext i8 %self.sroa.5.0.copyload to i64
  %ret.sroa.2.0.insert.shift = shl nuw nsw i64 %ret.sroa.2.0.insert.ext, 32
  %ret.sroa.0.0.insert.ext = zext i32 %self.sroa.4.0.copyload to i64
  %ret.sroa.0.0.insert.insert = or i64 %ret.sroa.2.0.insert.shift, %ret.sroa.0.0.insert.ext
  ret i64 %ret.sroa.0.0.insert.insert
}

; Function Attrs: inlinehint nounwind
define internal fastcc void @_ZN4core3ptr13drop_in_place17h187871343964cdfcE(%X** nocapture readonly) unnamed_addr #1 {
start:
  %1 = load %X*, %X** %0, align 8, !nonnull !1
  tail call fastcc void @_ZN4core3ptr13drop_in_place17h48c9e5bc21e188aaE(%X* nonnull %1)
  %2 = load %X*, %X** %0, align 8, !nonnull !1
  tail call fastcc void @_ZN5alloc4heap8box_free17h52fcf778f557f0b4E(%X* nonnull %2)
  ret void
}

; Function Attrs: inlinehint nounwind
define internal fastcc void @_ZN4core3ptr13drop_in_place17h31d8df590941a85fE(%"core::option::Option<E>"* readonly) unnamed_addr #1 {
start:
  %1 = getelementptr inbounds %"core::option::Option<E>", %"core::option::Option<E>"* %0, i64 0, i32 0
  %2 = load i64, i64* %1, align 8, !range !2
  %cond = icmp eq i64 %2, 0
  br i1 %cond, label %bb1, label %bb2

bb1:                                              ; preds = %start, %bb2
  ret void

bb2:                                              ; preds = %start
  %3 = getelementptr inbounds %"core::option::Option<E>", %"core::option::Option<E>"* %0, i64 0, i32 2
  %4 = bitcast [5 x i64]* %3 to %E*
  tail call fastcc void @_ZN4core3ptr13drop_in_place17hbf6cf7e003a035e6E(%E* %4)
  br label %bb1
}

; Function Attrs: inlinehint nounwind
define internal fastcc void @_ZN4core3ptr13drop_in_place17h48c9e5bc21e188aaE(%X* nocapture readonly) unnamed_addr #1 {
start:
  %1 = getelementptr inbounds %X, %X* %0, i64 0, i32 0
  %2 = load i64, i64* %1, align 8, !range !3
  %trunc = trunc i64 %2 to i2
  switch i2 %trunc, label %bb1 [
    i2 0, label %bb2
    i2 1, label %bb3
  ]

bb1:                                              ; preds = %bb2, %bb3, %start
  ret void

bb2:                                              ; preds = %start
  %3 = getelementptr inbounds %X, %X* %0, i64 0, i32 2
  %4 = bitcast [1 x i64]* %3 to %X**
  tail call fastcc void @_ZN4core3ptr13drop_in_place17h187871343964cdfcE(%X** %4)
  br label %bb1

bb3:                                              ; preds = %start
  %5 = getelementptr inbounds %X, %X* %0, i64 0, i32 2
  %6 = bitcast [1 x i64]* %5 to %X**
  tail call fastcc void @_ZN4core3ptr13drop_in_place17h187871343964cdfcE(%X** %6)
  br label %bb1
}

; Function Attrs: inlinehint nounwind
define internal fastcc void @_ZN4core3ptr13drop_in_place17hbf6cf7e003a035e6E(%E* readonly) unnamed_addr #1 {
start:
  %1 = getelementptr inbounds %E, %E* %0, i64 0, i32 0
  %2 = load i8, i8* %1, align 1, !range !4
  %cond = icmp eq i8 %2, 0
  br i1 %cond, label %bb1, label %slice_loop_body

bb1:                                              ; preds = %slice_loop_body, %start
  ret void

slice_loop_body:                                  ; preds = %start
  %3 = getelementptr inbounds %E, %E* %0, i64 0, i32 2
  %4 = bitcast [4 x i64]* %3 to %X*
  %5 = getelementptr inbounds %E, %E* %0, i64 0, i32 2, i64 2
  %6 = bitcast i64* %5 to %X*
  tail call fastcc void @_ZN4core3ptr13drop_in_place17h48c9e5bc21e188aaE(%X* %4)
  tail call fastcc void @_ZN4core3ptr13drop_in_place17h48c9e5bc21e188aaE(%X* %6)
  br label %bb1
}

; Function Attrs: norecurse nounwind readnone
define internal fastcc i64 @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hd676c2e417a31d76E"(i64) unnamed_addr #2 {
start:
  %ret.sroa.0.0.insert.insert = and i64 %0, 1099511627775
  ret i64 %ret.sroa.0.0.insert.insert
}

; Function Attrs: inlinehint nounwind
define internal fastcc void @_ZN5alloc4heap10deallocate17h0cf606d4dc6542b7E(i8*) unnamed_addr #1 {
start:
  tail call void @__rust_deallocate(i8* %0, i64 16, i64 8)
  ret void
}

; Function Attrs: inlinehint nounwind
define internal fastcc void @_ZN5alloc4heap8box_free17h52fcf778f557f0b4E(%X*) unnamed_addr #1 {
start:
  %1 = bitcast %X* %0 to i8*
  tail call fastcc void @_ZN5alloc4heap10deallocate17h0cf606d4dc6542b7E(i8* %1)
  ret void
}

; Function Attrs: norecurse nounwind
define internal fastcc void @"_ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$10from_error17h983ed059e5df6262E"(%"core::result::Result<(), (u32, u8)>"* noalias nocapture sret dereferenceable(12), i64) unnamed_addr #3 {
start:
  %abi_cast.sroa.0.0.extract.trunc = trunc i64 %1 to i32
  %abi_cast.sroa.4.0.extract.shift = lshr i64 %1, 32
  %abi_cast.sroa.4.0.extract.trunc = trunc i64 %abi_cast.sroa.4.0.extract.shift to i8
  %2 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %0, i64 0, i32 0
  store i32 1, i32* %2, align 4
  %3 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %0, i64 0, i32 2, i64 0
  store i32 %abi_cast.sroa.0.0.extract.trunc, i32* %3, align 4
  %4 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %0, i64 0, i32 2, i64 1
  %5 = bitcast i32* %4 to i8*
  store i8 %abi_cast.sroa.4.0.extract.trunc, i8* %5, align 4
  ret void
}

; Function Attrs: norecurse nounwind
define internal fastcc void @"_ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$12from_success17h4169241993d9016dE"(%"core::result::Result<(), (u32, u8)>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #3 {
start:
  %1 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %0, i64 0, i32 0
  store i32 0, i32* %1, align 4
  ret void
}

; Function Attrs: norecurse nounwind
define internal fastcc void @"_ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$9translate17had4b200d2ffbc2e9E"(%"core::result::Result<(), (u32, u8)>"* noalias nocapture sret dereferenceable(12), %"core::result::Result<(), (u32, u8)>"* noalias nocapture readonly dereferenceable(12)) unnamed_addr #3 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %self.sroa.0.0..sroa_idx = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %1, i64 0, i32 0
  %self.sroa.0.0.copyload = load i32, i32* %self.sroa.0.0..sroa_idx, align 4
  %cond = icmp eq i32 %self.sroa.0.0.copyload, 0
  br i1 %cond, label %bb3, label %bb4

bb3:                                              ; preds = %start
  tail call fastcc void @"_ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$12from_success17h4169241993d9016dE"(%"core::result::Result<(), (u32, u8)>"* noalias nocapture nonnull sret dereferenceable(12) %0)
  br label %bb5

bb4:                                              ; preds = %start
  %self.sroa.7.0..sroa_idx = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %1, i64 0, i32 2, i64 1
  %self.sroa.7.0..sroa_cast = bitcast i32* %self.sroa.7.0..sroa_idx to i8*
  %self.sroa.7.0.copyload = load i8, i8* %self.sroa.7.0..sroa_cast, align 4
  %self.sroa.6.0..sroa_idx6 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %1, i64 0, i32 2, i64 0
  %self.sroa.6.0.copyload = load i32, i32* %self.sroa.6.0..sroa_idx6, align 4
  %arg.sroa.2.0.insert.ext = zext i8 %self.sroa.7.0.copyload to i64
  %arg.sroa.2.0.insert.shift = shl nuw nsw i64 %arg.sroa.2.0.insert.ext, 32
  %arg.sroa.0.0.insert.ext = zext i32 %self.sroa.6.0.copyload to i64
  %arg.sroa.0.0.insert.insert = or i64 %arg.sroa.2.0.insert.shift, %arg.sroa.0.0.insert.ext
  tail call fastcc void @"_ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$10from_error17h983ed059e5df6262E"(%"core::result::Result<(), (u32, u8)>"* noalias nocapture nonnull sret dereferenceable(12) %0, i64 %arg.sroa.0.0.insert.insert)
  br label %bb5

bb5:                                              ; preds = %bb4, %bb3
  ret void
}

define internal void @_ZN2_14main17hb2ffcf74c477bad3E() unnamed_addr #4 {
start:
  %_1 = alloca %"core::result::Result<(), (u32, u8)>", align 8
  %0 = bitcast %"core::result::Result<(), (u32, u8)>"* %_1 to i8*
  call void @llvm.lifetime.start(i64 12, i8* nonnull %0)
  call fastcc void @_ZN2_11g17h62cbbaab7a2481a3E(%"core::result::Result<(), (u32, u8)>"* noalias nocapture nonnull sret dereferenceable(12) %_1)
  call void @llvm.lifetime.end(i64 12, i8* nonnull %0)
  ret void
}

define internal fastcc void @_ZN2_11g17h62cbbaab7a2481a3E(%"core::result::Result<(), (u32, u8)>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #4 {
start:
  %_28 = alloca %"core::option::Option<(u32, u8)>", align 8
  %_26 = alloca %"core::result::Result<(), (u32, u8)>", align 8
  %_25 = alloca %"core::result::Result<(), (u32, u8)>", align 8
  %_17 = alloca %"core::result::Result<(), (u32, u8)>", align 8
  %_16 = alloca %"core::result::Result<(), (u32, u8)>", align 8
  %infix_or_postfix = alloca %E, align 8
  %status = alloca %"core::option::Option<E>", align 8
  %y.sroa.7 = alloca [3 x i8], align 1
  %y.sroa.7.0..sroa_idx27 = getelementptr inbounds [3 x i8], [3 x i8]* %y.sroa.7, i64 0, i64 0
  call void @llvm.lifetime.start(i64 3, i8* nonnull %y.sroa.7.0..sroa_idx27)
  %1 = bitcast %"core::option::Option<E>"* %status to i8*
  %2 = getelementptr inbounds %"core::option::Option<E>", %"core::option::Option<E>"* %status, i64 0, i32 0
  %3 = bitcast %"core::option::Option<E>"* %status to { i64, [0 x i8], %E, [0 x i8] }*
  %_8.sroa.0.0..sroa_idx = getelementptr inbounds %"core::option::Option<E>", %"core::option::Option<E>"* %status, i64 0, i32 2
  %4 = bitcast [5 x i64]* %_8.sroa.0.0..sroa_idx to i8*
  %_8.sroa.4.0..sroa_idx = getelementptr inbounds { i64, [0 x i8], %E, [0 x i8] }, { i64, [0 x i8], %E, [0 x i8] }* %3, i64 0, i32 2, i32 1, i64 0
  %5 = getelementptr inbounds %"core::option::Option<E>", %"core::option::Option<E>"* %status, i64 0, i32 0
  %6 = getelementptr inbounds %E, %E* %infix_or_postfix, i64 0, i32 0
  %7 = getelementptr inbounds %"core::option::Option<E>", %"core::option::Option<E>"* %status, i64 0, i32 2
  %8 = bitcast [5 x i64]* %7 to i8*
  %9 = bitcast %"core::result::Result<(), (u32, u8)>"* %_16 to i8*
  %10 = bitcast %"core::result::Result<(), (u32, u8)>"* %_17 to i8*
  %11 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %_17, i64 0, i32 0
  %12 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %_16, i64 0, i32 0
  %13 = getelementptr inbounds %E, %E* %infix_or_postfix, i64 0, i32 0
  %14 = getelementptr inbounds %E, %E* %infix_or_postfix, i64 0, i32 0
  %15 = getelementptr inbounds %"core::option::Option<E>", %"core::option::Option<E>"* %status, i64 0, i32 2
  %16 = bitcast [5 x i64]* %15 to %E*
  %17 = bitcast %"core::result::Result<(), (u32, u8)>"* %_25 to i8*
  %18 = bitcast %"core::result::Result<(), (u32, u8)>"* %_26 to i8*
  %19 = bitcast %"core::option::Option<(u32, u8)>"* %_28 to i8*
  %y.sroa.0.0..sroa_idx = getelementptr inbounds %"core::option::Option<(u32, u8)>", %"core::option::Option<(u32, u8)>"* %_28, i64 0, i32 0
  %y.sroa.5.0..sroa_idx23 = getelementptr inbounds %"core::option::Option<(u32, u8)>", %"core::option::Option<(u32, u8)>"* %_28, i64 0, i32 2, i64 0
  %y.sroa.6.0..sroa_idx = getelementptr inbounds %"core::option::Option<(u32, u8)>", %"core::option::Option<(u32, u8)>"* %_28, i64 0, i32 2, i64 1
  %y.sroa.6.0..sroa_cast = bitcast i32* %y.sroa.6.0..sroa_idx to i8*
  %y.sroa.7.0..sroa_raw_idx = getelementptr inbounds i8, i8* %19, i64 9
  %20 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %_26, i64 0, i32 0
  %21 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %_26, i64 0, i32 2, i64 0
  %22 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %_26, i64 0, i32 2, i64 1
  %23 = bitcast i32* %22 to i8*
  %24 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %_25, i64 0, i32 0
  %25 = getelementptr inbounds %"core::option::Option<E>", %"core::option::Option<E>"* %status, i64 0, i32 0
  br label %bb1

bb1:                                              ; preds = %bb25, %start
  %y.sroa.0.0 = phi i32 [ 0, %start ], [ %y.sroa.0.14, %bb25 ]
  %pt.sroa.0.0 = phi i1 [ true, %start ], [ false, %bb25 ]
  %_35.0 = phi i8 [ 0, %start ], [ %_35.2, %bb25 ]
  call void @llvm.lifetime.start(i64 48, i8* nonnull %1)
  br i1 %pt.sroa.0.0, label %bb4, label %bb4.thread

bb4.thread:                                       ; preds = %bb1
  store i64 0, i64* %25, align 8
  call void @llvm.lifetime.start(i64 12, i8* nonnull %17)
  call void @llvm.lifetime.start(i64 12, i8* nonnull %18)
  call void @llvm.lifetime.start(i64 12, i8* nonnull %19)
  store i32 %y.sroa.0.0, i32* %y.sroa.0.0..sroa_idx, align 8
  store i32 1, i32* %y.sroa.5.0..sroa_idx23, align 4
  store i8 0, i8* %y.sroa.6.0..sroa_cast, align 4
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %y.sroa.7.0..sroa_raw_idx, i8* nonnull %y.sroa.7.0..sroa_idx27, i64 3, i32 1, i1 false)
  %26 = call fastcc i64 @"_ZN38_$LT$core..option..Option$LT$T$GT$$GT$6unwrap17hdbd5a86115aeeb48E"(%"core::option::Option<(u32, u8)>"* noalias nocapture nonnull dereferenceable(12) %_28)
  %abi_cast1.sroa.0.0.extract.trunc = trunc i64 %26 to i32
  %abi_cast1.sroa.4.0.extract.shift = lshr i64 %26, 32
  %abi_cast1.sroa.4.0.extract.trunc = trunc i64 %abi_cast1.sroa.4.0.extract.shift to i8
  call void @llvm.lifetime.end(i64 12, i8* nonnull %19)
  store i32 1, i32* %20, align 8
  store i32 %abi_cast1.sroa.0.0.extract.trunc, i32* %21, align 4
  store i8 %abi_cast1.sroa.4.0.extract.trunc, i8* %23, align 4
  call fastcc void @"_ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$9translate17had4b200d2ffbc2e9E"(%"core::result::Result<(), (u32, u8)>"* noalias nocapture nonnull sret dereferenceable(12) %_25, %"core::result::Result<(), (u32, u8)>"* noalias nocapture nonnull dereferenceable(12) %_26)
  call void @llvm.lifetime.end(i64 12, i8* nonnull %18)
  %27 = load i32, i32* %24, align 8, !range !5
  %cond9 = icmp eq i32 %27, 0
  br i1 %cond9, label %bb7, label %bb32

bb4:                                              ; preds = %bb1
  store i64 1, i64* %2, align 8
  store i8 0, i8* %4, align 8
  store i8 0, i8* %_8.sroa.4.0..sroa_idx, align 1
  call void @llvm.lifetime.start(i64 40, i8* nonnull %6)
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull %6, i8* nonnull %8, i64 40, i32 8, i1 false)
  %28 = load i8, i8* %6, align 8, !range !4
  %cond14 = icmp eq i8 %28, 0
  br i1 %cond14, label %bb9, label %bb27

bb7:                                              ; preds = %bb4.thread
  call void @llvm.lifetime.end(i64 12, i8* nonnull %17)
  %29 = and i8 %_35.0, 1
  %30 = icmp eq i8 %29, 0
  br i1 %30, label %bb24, label %bb28

bb9:                                              ; preds = %bb4
  call void @llvm.lifetime.start(i64 12, i8* nonnull %9)
  call void @llvm.lifetime.start(i64 12, i8* nonnull %10)
  store i32 0, i32* %11, align 8
  call fastcc void @"_ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$9translate17had4b200d2ffbc2e9E"(%"core::result::Result<(), (u32, u8)>"* noalias nocapture nonnull sret dereferenceable(12) %_16, %"core::result::Result<(), (u32, u8)>"* noalias nocapture nonnull dereferenceable(12) %_17)
  call void @llvm.lifetime.end(i64 12, i8* nonnull %10)
  %31 = load i32, i32* %12, align 8, !range !5
  %cond15 = icmp eq i32 %31, 0
  br i1 %cond15, label %bb12, label %bb32.thread

bb12:                                             ; preds = %bb9
  call void @llvm.lifetime.end(i64 12, i8* nonnull %9)
  br label %bb28

bb32.thread:                                      ; preds = %bb9
  %32 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %_16, i64 0, i32 2, i64 0
  %33 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %_16, i64 0, i32 2, i64 1
  %34 = bitcast i32* %33 to i8*
  %35 = load i32, i32* %32, align 4
  %36 = load i8, i8* %34, align 4
  %arg5.sroa.2.0.insert.ext = zext i8 %36 to i64
  %arg5.sroa.2.0.insert.shift = shl nuw nsw i64 %arg5.sroa.2.0.insert.ext, 32
  %arg5.sroa.0.0.insert.ext = zext i32 %35 to i64
  %arg5.sroa.0.0.insert.insert = or i64 %arg5.sroa.2.0.insert.shift, %arg5.sroa.0.0.insert.ext
  %37 = call fastcc i64 @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hd676c2e417a31d76E"(i64 %arg5.sroa.0.0.insert.insert)
  %arg8.sroa.0.0.insert.insert = and i64 %37, 1099511627775
  call fastcc void @"_ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$10from_error17h983ed059e5df6262E"(%"core::result::Result<(), (u32, u8)>"* noalias nocapture nonnull sret dereferenceable(12) %0, i64 %arg8.sroa.0.0.insert.insert)
  call void @llvm.lifetime.end(i64 12, i8* nonnull %9)
  br label %bb31

bb16:                                             ; preds = %bb32, %bb31, %bb30
  %_38.29 = phi i1 [ true, %bb32 ], [ %_38.28, %bb31 ], [ %_38.28, %bb30 ]
  %38 = getelementptr inbounds %E, %E* %infix_or_postfix, i64 0, i32 0
  call void @llvm.lifetime.end(i64 40, i8* nonnull %38)
  %39 = load i64, i64* %5, align 8, !range !2
  %cond10 = icmp eq i64 %39, 1
  br i1 %cond10, label %bb34, label %bb33

bb17:                                             ; preds = %bb35, %bb34, %bb33
  call void @llvm.lifetime.end(i64 48, i8* nonnull %1)
  call void @llvm.lifetime.end(i64 3, i8* nonnull %y.sroa.7.0..sroa_idx27)
  ret void

bb24:                                             ; preds = %bb7, %bb28, %bb27
  %_38.17 = phi i1 [ %_38.16.ph, %bb28 ], [ %_38.1620, %bb27 ], [ true, %bb7 ]
  %y.sroa.0.14 = phi i32 [ %y.sroa.0.13.ph, %bb28 ], [ %y.sroa.0.1321, %bb27 ], [ %y.sroa.0.0, %bb7 ]
  %_35.2 = phi i8 [ %_35.15.ph, %bb28 ], [ 0, %bb27 ], [ %_35.0, %bb7 ]
  call void @llvm.lifetime.end(i64 40, i8* nonnull %14)
  %40 = load i64, i64* %5, align 8, !range !2
  %cond12 = icmp eq i64 %40, 1
  br i1 %cond12, label %bb37, label %bb36

bb25:                                             ; preds = %bb38, %bb37, %bb36
  call void @llvm.lifetime.end(i64 48, i8* nonnull %1)
  br label %bb1

bb27:                                             ; preds = %bb4, %bb28
  %y.sroa.0.1321 = phi i32 [ %y.sroa.0.13.ph, %bb28 ], [ %y.sroa.0.0, %bb4 ]
  %_38.1620 = phi i1 [ %_38.16.ph, %bb28 ], [ false, %bb4 ]
  call fastcc void @_ZN4core3ptr13drop_in_place17hbf6cf7e003a035e6E(%E* nonnull %infix_or_postfix)
  br label %bb24

bb28:                                             ; preds = %bb12, %bb7
  %_38.16.ph = phi i1 [ false, %bb12 ], [ true, %bb7 ]
  %_35.15.ph = phi i8 [ 1, %bb12 ], [ %_35.0, %bb7 ]
  %y.sroa.0.13.ph = phi i32 [ 1, %bb12 ], [ %y.sroa.0.0, %bb7 ]
  %.pr18 = load i8, i8* %13, align 8
  %cond13 = icmp eq i8 %.pr18, 0
  br i1 %cond13, label %bb24, label %bb27

bb30:                                             ; preds = %bb31
  call fastcc void @_ZN4core3ptr13drop_in_place17hbf6cf7e003a035e6E(%E* nonnull %infix_or_postfix)
  br label %bb16

bb31:                                             ; preds = %bb32.thread, %bb32
  %_38.28 = phi i1 [ false, %bb32.thread ], [ true, %bb32 ]
  %41 = getelementptr inbounds %E, %E* %infix_or_postfix, i64 0, i32 0
  %42 = load i8, i8* %41, align 8, !range !4
  %cond11 = icmp eq i8 %42, 0
  br i1 %cond11, label %bb16, label %bb30

bb32:                                             ; preds = %bb4.thread
  %43 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %_25, i64 0, i32 2, i64 0
  %44 = getelementptr inbounds %"core::result::Result<(), (u32, u8)>", %"core::result::Result<(), (u32, u8)>"* %_25, i64 0, i32 2, i64 1
  %45 = bitcast i32* %44 to i8*
  %46 = load i32, i32* %43, align 4
  %47 = load i8, i8* %45, align 4
  %arg.sroa.2.0.insert.ext = zext i8 %47 to i64
  %arg.sroa.2.0.insert.shift = shl nuw nsw i64 %arg.sroa.2.0.insert.ext, 32
  %arg.sroa.0.0.insert.ext = zext i32 %46 to i64
  %arg.sroa.0.0.insert.insert = or i64 %arg.sroa.2.0.insert.shift, %arg.sroa.0.0.insert.ext
  %48 = call fastcc i64 @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hd676c2e417a31d76E"(i64 %arg.sroa.0.0.insert.insert)
  %arg4.sroa.0.0.insert.insert = and i64 %48, 1099511627775
  call fastcc void @"_ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$10from_error17h983ed059e5df6262E"(%"core::result::Result<(), (u32, u8)>"* noalias nocapture nonnull sret dereferenceable(12) %0, i64 %arg4.sroa.0.0.insert.insert)
  call void @llvm.lifetime.end(i64 12, i8* nonnull %17)
  %49 = and i8 %_35.0, 1
  %50 = icmp eq i8 %49, 0
  br i1 %50, label %bb16, label %bb31

bb33:                                             ; preds = %bb16
  call fastcc void @_ZN4core3ptr13drop_in_place17h31d8df590941a85fE(%"core::option::Option<E>"* nonnull %status)
  br label %bb17

bb34:                                             ; preds = %bb16
  br i1 %_38.29, label %bb35, label %bb17

bb35:                                             ; preds = %bb34
  %51 = getelementptr inbounds %"core::option::Option<E>", %"core::option::Option<E>"* %status, i64 0, i32 2
  %52 = bitcast [5 x i64]* %51 to %E*
  call fastcc void @_ZN4core3ptr13drop_in_place17hbf6cf7e003a035e6E(%E* %52)
  br label %bb17

bb36:                                             ; preds = %bb24
  call fastcc void @_ZN4core3ptr13drop_in_place17h31d8df590941a85fE(%"core::option::Option<E>"* nonnull %status)
  br label %bb25

bb37:                                             ; preds = %bb24
  br i1 %_38.17, label %bb38, label %bb25

bb38:                                             ; preds = %bb37
  call fastcc void @_ZN4core3ptr13drop_in_place17hbf6cf7e003a035e6E(%E* nonnull %16)
  br label %bb25
}

declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #4

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.start(i64, i8* nocapture) #5

; Function Attrs: argmemonly nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i32, i1) #5

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.end(i64, i8* nocapture) #5

; Function Attrs: cold noinline noreturn
declare void @_ZN4core9panicking5panic17hafbe89720e5223c3E({ %str_slice, [0 x i8], %str_slice, [0 x i8], i32, [4 x i8] }* noalias readonly dereferenceable(40)) unnamed_addr #6

; Function Attrs: nounwind
declare void @__rust_deallocate(i8*, i64, i64) unnamed_addr #7

define i64 @main(i64, i8**) unnamed_addr #4 {
top:
  %2 = tail call i64 @_ZN3std2rt10lang_start17hbda94898b2b88ae4E(i8* bitcast (void ()* @_ZN2_14main17hb2ffcf74c477bad3E to i8*), i64 %0, i8** %1)
  ret i64 %2
}

declare i64 @_ZN3std2rt10lang_start17hbda94898b2b88ae4E(i8*, i64, i8**) unnamed_addr #4

attributes #0 = { inlinehint "no-frame-pointer-elim"="true" }
attributes #1 = { inlinehint nounwind "no-frame-pointer-elim"="true" }
attributes #2 = { norecurse nounwind readnone "no-frame-pointer-elim"="true" }
attributes #3 = { norecurse nounwind "no-frame-pointer-elim"="true" }
attributes #4 = { "no-frame-pointer-elim"="true" }
attributes #5 = { argmemonly nounwind }
attributes #6 = { cold noinline noreturn "no-frame-pointer-elim"="true" }
attributes #7 = { nounwind "no-frame-pointer-elim"="true" }

!llvm.module.flags = !{!0}

!0 = !{i32 1, !"PIE Level", i32 2}
!1 = !{}
!2 = !{i64 0, i64 2}
!3 = !{i64 0, i64 3}
!4 = !{i8 0, i8 2}
!5 = !{i32 0, i32 2}
