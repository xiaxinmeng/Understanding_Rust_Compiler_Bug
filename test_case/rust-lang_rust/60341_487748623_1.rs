llvm
; thread_local::get_the_thread_local
; Function Attrs: uwtable
define void @_ZN12thread_local20get_the_thread_local17h709f40512ac1fd32E(%"alloc::string::String"* noalias nocapture sret dereferenceable(24)) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %value.i.i.i = alloca %"alloc::string::String", align 16
  %_15.i.i = alloca %"alloc::string::String", align 8
  %_3.sroa.6.i = alloca [2 x i64], align 8
  %_3.sroa.6.0.sroa_cast9.i = bitcast [2 x i64]* %_3.sroa.6.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %_3.sroa.6.0.sroa_cast9.i)
  %.val.i.i.i.i = load i8, i8* getelementptr inbounds (<{ [32 x i8] }>, <{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h53cabb331a543277E, i64 0, i32 0, i64 25), align 1, !noalias !25
  %1 = icmp eq i8 %.val.i.i.i.i, 0
  br i1 %1, label %bb7.i.i.i.i, label %bb4.i.i

bb7.i.i.i.i:                                      ; preds = %start
  %.val.i.i.i.i.i = load i8, i8* getelementptr inbounds (<{ [32 x i8] }>, <{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h53cabb331a543277E, i64 0, i32 0, i64 24), align 8, !noalias !25
  %2 = icmp eq i8 %.val.i.i.i.i.i, 0
  br i1 %2, label %bb7.i.i.i.i.i, label %bb11.i.i

bb7.i.i.i.i.i:                                    ; preds = %bb7.i.i.i.i
; call std::sys::unix::fast_thread_local::register_dtor
  call void @_ZN3std3sys4unix17fast_thread_local13register_dtor17hea5fd2f6f1030afcE(i8* getelementptr inbounds (<{ [32 x i8] }>, <{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h53cabb331a543277E, i64 0, i32 0, i64 0), void (i8*)* nonnull @_ZN3std6thread5local4fast13destroy_value17h328a717a41c9449aE), !noalias !25
  store i8 1, i8* getelementptr inbounds (<{ [32 x i8] }>, <{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h53cabb331a543277E, i64 0, i32 0, i64 24), align 8, !noalias !25
  br label %bb11.i.i

bb11.i.i:                                         ; preds = %bb7.i.i.i.i.i, %bb7.i.i.i.i
  %3 = bitcast %"alloc::string::String"* %_15.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %3), !noalias !25
  %4 = load {}*, {}** bitcast (<{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h53cabb331a543277E to {}**), align 8, !noalias !25
  %5 = icmp eq {}* %4, null
  br i1 %5, label %bb13.i.i, label %bb15.i.i

bb13.i.i:                                         ; preds = %bb11.i.i
  %6 = bitcast %"alloc::string::String"* %value.i.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %6), !noalias !25
; call thread_local::init
  call fastcc void @_ZN12thread_local4init17h88c40ced9b6cbe50E(%"alloc::string::String"* noalias nocapture nonnull dereferenceable(24) %value.i.i.i) #7, !noalias !25
  %7 = bitcast %"alloc::string::String"* %value.i.i.i to <2 x i64>*
  %8 = load <2 x i64>, <2 x i64>* %7, align 16, !noalias !25
  %_11.sroa.0.sroa.5.0..sroa_idx21.i.i.i = getelementptr inbounds %"alloc::string::String", %"alloc::string::String"* %value.i.i.i, i64 0, i32 1, i32 3
  %_11.sroa.0.sroa.5.0.copyload.i.i.i = load i64, i64* %_11.sroa.0.sroa.5.0..sroa_idx21.i.i.i, align 16, !noalias !25
  %z.i.i.i.sroa.0.0.copyload.i.i.i = load {}*, {}** bitcast (<{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h53cabb331a543277E to {}**), align 8, !noalias !30
  %z.i.i.i.sroa.4.0.copyload.i.i.i = load i64, i64* bitcast (i8* getelementptr inbounds (<{ [32 x i8] }>, <{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h53cabb331a543277E, i64 0, i32 0, i64 8) to i64*), align 8, !noalias !30
  store <2 x i64> %8, <2 x i64>* bitcast (<{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h53cabb331a543277E to <2 x i64>*), align 8, !noalias !34
  store i64 %_11.sroa.0.sroa.5.0.copyload.i.i.i, i64* bitcast (i8* getelementptr inbounds (<{ [32 x i8] }>, <{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h53cabb331a543277E, i64 0, i32 0, i64 16) to i64*), align 8, !noalias !34
  %9 = icmp eq {}* %z.i.i.i.sroa.0.0.copyload.i.i.i, null
  %10 = icmp eq i64 %z.i.i.i.sroa.4.0.copyload.i.i.i, 0
  %or.cond.i.i.i = or i1 %9, %10
  br i1 %or.cond.i.i.i, label %"_ZN3std6thread5local17LocalKey$LT$T$GT$4init17hf02e3e1800d44f2aE.exit.i.i", label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb13.i.i
  %11 = bitcast {}* %z.i.i.i.sroa.0.0.copyload.i.i.i to i8*
  tail call void @__rust_dealloc(i8* nonnull %11, i64 %z.i.i.i.sroa.4.0.copyload.i.i.i, i64 1) #7, !noalias !25
  br label %"_ZN3std6thread5local17LocalKey$LT$T$GT$4init17hf02e3e1800d44f2aE.exit.i.i"

"_ZN3std6thread5local17LocalKey$LT$T$GT$4init17hf02e3e1800d44f2aE.exit.i.i": ; preds = %bb4.i.i.i.i.i.i.i.i.i, %bb13.i.i
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %6), !noalias !25
  br label %bb15.i.i

bb15.i.i:                                         ; preds = %"_ZN3std6thread5local17LocalKey$LT$T$GT$4init17hf02e3e1800d44f2aE.exit.i.i", %bb11.i.i
; call <alloc::string::String as core::clone::Clone>::clone
  call void @"_ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17h5e0882d4896c9b5eE"(%"alloc::string::String"* noalias nocapture nonnull sret dereferenceable(24) %_15.i.i, %"alloc::string::String"* noalias nonnull readonly align 8 dereferenceable(24) bitcast (<{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h53cabb331a543277E to %"alloc::string::String"*)), !noalias !25
  %_3.sroa.0.0..sroa_cast.i = bitcast %"alloc::string::String"* %_15.i.i to {}**
  %_3.sroa.0.0.copyload.i = load {}*, {}** %_3.sroa.0.0..sroa_cast.i, align 8, !noalias !35
  %_3.sroa.6.0..sroa_idx.i = getelementptr inbounds %"alloc::string::String", %"alloc::string::String"* %_15.i.i, i64 0, i32 1, i32 1, i32 1
  %_3.sroa.6.0..sroa_cast.i = bitcast i64* %_3.sroa.6.0..sroa_idx.i to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 %_3.sroa.6.0.sroa_cast9.i, i8* nonnull align 8 %_3.sroa.6.0..sroa_cast.i, i64 16, i1 false), !noalias !35
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %3), !noalias !25
  %12 = icmp eq {}* %_3.sroa.0.0.copyload.i, null
  br i1 %12, label %bb4.i.i, label %"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17hf1fd224675692f45E.exit"

bb4.i.i:                                          ; preds = %bb15.i.i, %start
; call core::result::unwrap_failed
  tail call fastcc void @_ZN4core6result13unwrap_failed17hf64e0430bdaf4bcaE(), !noalias !36
  unreachable

"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17hf1fd224675692f45E.exit": ; preds = %bb15.i.i
  %_3.sroa.0.0..sroa_cast2.i = bitcast %"alloc::string::String"* %0 to {}**
  store {}* %_3.sroa.0.0.copyload.i, {}** %_3.sroa.0.0..sroa_cast2.i, align 8, !alias.scope !36
  %_3.sroa.6.0..sroa_idx6.i = getelementptr inbounds %"alloc::string::String", %"alloc::string::String"* %0, i64 0, i32 1, i32 1, i32 1
  %_3.sroa.6.0..sroa_cast7.i = bitcast i64* %_3.sroa.6.0..sroa_idx6.i to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 %_3.sroa.6.0..sroa_cast7.i, i8* nonnull align 8 %_3.sroa.6.0.sroa_cast9.i, i64 16, i1 false), !alias.scope !40
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %_3.sroa.6.0.sroa_cast9.i)
  ret void
}
