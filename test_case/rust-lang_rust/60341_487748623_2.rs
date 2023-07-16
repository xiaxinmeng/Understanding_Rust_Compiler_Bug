llvm
; thread_local::get_the_thread_local
; Function Attrs: uwtable
define void @_ZN12thread_local20get_the_thread_local17hceac182570e9506dE(%"alloc::string::String"* noalias nocapture sret dereferenceable(24)) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %value.i.i.i = alloca %"alloc::string::String", align 16
  %self.i.i.i.i = alloca %"std::thread::local::fast::Key<alloc::string::String>"*, align 8
  %_15.i.i = alloca %"alloc::string::String", align 8
  %_3.sroa.6.i = alloca [2 x i64], align 8
  %_3.sroa.6.0.sroa_cast9.i = bitcast [2 x i64]* %_3.sroa.6.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %_3.sroa.6.0.sroa_cast9.i)
  %self.i.i.i.i.0.sroa_cast = bitcast %"std::thread::local::fast::Key<alloc::string::String>"** %self.i.i.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %self.i.i.i.i.0.sroa_cast)
  store %"std::thread::local::fast::Key<alloc::string::String>"* bitcast (<{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h1f31f65c90417afeE to %"std::thread::local::fast::Key<alloc::string::String>"*), %"std::thread::local::fast::Key<alloc::string::String>"** %self.i.i.i.i, align 8, !noalias !25
  %self.i.i.i.i.0.self.i.i.i.0.self.i.i.0.self.i.0.self.0..i.i.i.i = load volatile %"std::thread::local::fast::Key<alloc::string::String>"*, %"std::thread::local::fast::Key<alloc::string::String>"** %self.i.i.i.i, align 8, !noalias !25, !nonnull !0
  %1 = getelementptr inbounds %"std::thread::local::fast::Key<alloc::string::String>", %"std::thread::local::fast::Key<alloc::string::String>"* %self.i.i.i.i.0.self.i.i.i.0.self.i.i.0.self.i.0.self.0..i.i.i.i, i64 0, i32 5
  %.val.i.i.i.i = load i8, i8* %1, align 1, !noalias !25
  %2 = icmp eq i8 %.val.i.i.i.i, 0
  br i1 %2, label %bb8.i.i.i.i, label %"_ZN3std6thread5local17LocalKey$LT$T$GT$8try_with17ha49ffcfe07458aa0E.exit.thread.i"

bb8.i.i.i.i:                                      ; preds = %start
  %3 = getelementptr inbounds %"std::thread::local::fast::Key<alloc::string::String>", %"std::thread::local::fast::Key<alloc::string::String>"* %self.i.i.i.i.0.self.i.i.i.0.self.i.i.0.self.i.0.self.0..i.i.i.i, i64 0, i32 3
  %.val.i.i.i.i.i = load i8, i8* %3, align 8, !noalias !25
  %4 = icmp eq i8 %.val.i.i.i.i.i, 0
  br i1 %4, label %bb7.i.i.i.i.i, label %bb11.i.i

bb7.i.i.i.i.i:                                    ; preds = %bb8.i.i.i.i
  %5 = bitcast %"std::thread::local::fast::Key<alloc::string::String>"* %self.i.i.i.i.0.self.i.i.i.0.self.i.i.0.self.i.0.self.0..i.i.i.i to i8*
; call std::sys::unix::fast_thread_local::register_dtor
  call void @_ZN3std3sys4unix17fast_thread_local13register_dtor17ha1a2dbcd05fabbafE(i8* nonnull %5, void (i8*)* nonnull @_ZN3std6thread5local4fast13destroy_value17haead139569d8605fE), !noalias !25
  store i8 1, i8* %3, align 8, !noalias !25
  br label %bb11.i.i

"_ZN3std6thread5local17LocalKey$LT$T$GT$8try_with17ha49ffcfe07458aa0E.exit.thread.i": ; preds = %start
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %self.i.i.i.i.0.sroa_cast)
  br label %bb4.i.i

bb11.i.i:                                         ; preds = %bb7.i.i.i.i.i, %bb8.i.i.i.i
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %self.i.i.i.i.0.sroa_cast)
  %6 = bitcast %"alloc::string::String"* %_15.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %6), !noalias !25
  %7 = bitcast %"std::thread::local::fast::Key<alloc::string::String>"* %self.i.i.i.i.0.self.i.i.i.0.self.i.i.0.self.i.0.self.0..i.i.i.i to {}**
  %8 = load {}*, {}** %7, align 8, !noalias !25
  %9 = icmp eq {}* %8, null
  br i1 %9, label %bb13.i.i, label %bb15.i.i

bb13.i.i:                                         ; preds = %bb11.i.i
  %10 = bitcast %"alloc::string::String"* %value.i.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %10), !noalias !25
; call thread_local::init
  call fastcc void @_ZN12thread_local4init17h08fde1884c30376bE(%"alloc::string::String"* noalias nocapture nonnull dereferenceable(24) %value.i.i.i) #6, !noalias !25
  %11 = bitcast %"alloc::string::String"* %value.i.i.i to <2 x i64>*
  %12 = load <2 x i64>, <2 x i64>* %11, align 16, !noalias !25
  %_11.sroa.0.sroa.5.0..sroa_idx21.i.i.i = getelementptr inbounds %"alloc::string::String", %"alloc::string::String"* %value.i.i.i, i64 0, i32 1, i32 3
  %_11.sroa.0.sroa.5.0.copyload.i.i.i = load i64, i64* %_11.sroa.0.sroa.5.0..sroa_idx21.i.i.i, align 16, !noalias !25
  %z.i.i.i.sroa.0.0.copyload.i.i.i = load {}*, {}** %7, align 8, !noalias !30
  %13 = getelementptr inbounds %"std::thread::local::fast::Key<alloc::string::String>", %"std::thread::local::fast::Key<alloc::string::String>"* %self.i.i.i.i.0.self.i.i.i.0.self.i.i.0.self.i.0.self.0..i.i.i.i, i64 0, i32 1, i32 1, i32 2, i64 0
  %z.i.i.i.sroa.4.0.copyload.i.i.i = load i64, i64* %13, align 8, !noalias !30
  %14 = getelementptr inbounds %"std::thread::local::fast::Key<alloc::string::String>", %"std::thread::local::fast::Key<alloc::string::String>"* %self.i.i.i.i.0.self.i.i.i.0.self.i.i.0.self.i.0.self.0..i.i.i.i, i64 0, i32 1, i32 1, i32 2, i64 1
  %15 = bitcast %"std::thread::local::fast::Key<alloc::string::String>"* %self.i.i.i.i.0.self.i.i.i.0.self.i.i.0.self.i.0.self.0..i.i.i.i to <2 x i64>*
  store <2 x i64> %12, <2 x i64>* %15, align 8, !noalias !34
  store i64 %_11.sroa.0.sroa.5.0.copyload.i.i.i, i64* %14, align 8, !noalias !34
  %16 = icmp eq {}* %z.i.i.i.sroa.0.0.copyload.i.i.i, null
  %17 = icmp eq i64 %z.i.i.i.sroa.4.0.copyload.i.i.i, 0
  %or.cond.i.i.i = or i1 %16, %17
  br i1 %or.cond.i.i.i, label %"_ZN3std6thread5local17LocalKey$LT$T$GT$4init17h4a7d4a29fdf48028E.exit.i.i", label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb13.i.i
  %18 = bitcast {}* %z.i.i.i.sroa.0.0.copyload.i.i.i to i8*
  tail call void @__rust_dealloc(i8* nonnull %18, i64 %z.i.i.i.sroa.4.0.copyload.i.i.i, i64 1) #6, !noalias !25
  br label %"_ZN3std6thread5local17LocalKey$LT$T$GT$4init17h4a7d4a29fdf48028E.exit.i.i"

"_ZN3std6thread5local17LocalKey$LT$T$GT$4init17h4a7d4a29fdf48028E.exit.i.i": ; preds = %bb4.i.i.i.i.i.i.i.i.i, %bb13.i.i
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %10), !noalias !25
  br label %bb15.i.i

bb15.i.i:                                         ; preds = %"_ZN3std6thread5local17LocalKey$LT$T$GT$4init17h4a7d4a29fdf48028E.exit.i.i", %bb11.i.i
  %_19.0.i.i = bitcast %"std::thread::local::fast::Key<alloc::string::String>"* %self.i.i.i.i.0.self.i.i.i.0.self.i.i.0.self.i.0.self.0..i.i.i.i to %"alloc::string::String"*
; call <alloc::string::String as core::clone::Clone>::clone
  call void @"_ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17h9234dcb674122143E"(%"alloc::string::String"* noalias nocapture nonnull sret dereferenceable(24) %_15.i.i, %"alloc::string::String"* noalias nonnull readonly align 8 dereferenceable(24) %_19.0.i.i), !noalias !25
  %_3.sroa.0.0..sroa_cast.i = bitcast %"alloc::string::String"* %_15.i.i to {}**
  %_3.sroa.0.0.copyload.i = load {}*, {}** %_3.sroa.0.0..sroa_cast.i, align 8, !noalias !35
  %_3.sroa.6.0..sroa_idx.i = getelementptr inbounds %"alloc::string::String", %"alloc::string::String"* %_15.i.i, i64 0, i32 1, i32 1, i32 1
  %_3.sroa.6.0..sroa_cast.i = bitcast i64* %_3.sroa.6.0..sroa_idx.i to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 %_3.sroa.6.0.sroa_cast9.i, i8* nonnull align 8 %_3.sroa.6.0..sroa_cast.i, i64 16, i1 false), !noalias !35
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %6), !noalias !25
  %19 = icmp eq {}* %_3.sroa.0.0.copyload.i, null
  br i1 %19, label %bb4.i.i, label %"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17h0b7fbfbefd581825E.exit"

bb4.i.i:                                          ; preds = %bb15.i.i, %"_ZN3std6thread5local17LocalKey$LT$T$GT$8try_with17ha49ffcfe07458aa0E.exit.thread.i"
; call core::result::unwrap_failed
  tail call fastcc void @_ZN4core6result13unwrap_failed17hcfbdd175a3d3f832E(), !noalias !36
  unreachable

"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17h0b7fbfbefd581825E.exit": ; preds = %bb15.i.i
  %_3.sroa.0.0..sroa_cast2.i = bitcast %"alloc::string::String"* %0 to {}**
  store {}* %_3.sroa.0.0.copyload.i, {}** %_3.sroa.0.0..sroa_cast2.i, align 8, !alias.scope !36
  %_3.sroa.6.0..sroa_idx6.i = getelementptr inbounds %"alloc::string::String", %"alloc::string::String"* %0, i64 0, i32 1, i32 1, i32 1
  %_3.sroa.6.0..sroa_cast7.i = bitcast i64* %_3.sroa.6.0..sroa_idx6.i to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 %_3.sroa.6.0..sroa_cast7.i, i8* nonnull align 8 %_3.sroa.6.0.sroa_cast9.i, i64 16, i1 false), !alias.scope !40
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %_3.sroa.6.0.sroa_cast9.i)
  ret void
}
