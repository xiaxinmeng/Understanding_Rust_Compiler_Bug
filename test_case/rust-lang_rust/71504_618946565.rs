
@_ZN6memchr3x867memrchr2FN17h5bc0baf729fbe004E = external dllimport local_unnamed_addr global %"core::sync::atomic::AtomicPtr<()>"

; globset::pathutil::file_name
; Function Attrs: uwtable
define void @_ZN7globset8pathutil9file_name17hbcd7b89f83326b91E(%"core::option::Option<alloc::borrow::Cow<[u8]>>"* noalias nocapture sret dereferenceable(32), %"alloc::borrow::Cow<[u8]>"* noalias nocapture readonly align 8 dereferenceable(32) %path) unnamed_addr #0 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %vector.i.i.i = alloca %"alloc::vec::Vec<u8>", align 8
  %path1 = alloca %"alloc::vec::Vec<u8>", align 8
  %1 = getelementptr inbounds %"alloc::borrow::Cow<[u8]>", %"alloc::borrow::Cow<[u8]>"* %path, i64 0, i32 0, i64 0
  %_3.i = load i64, i64* %1, align 8, !range !2, !alias.scope !5129
  %switch.i = icmp eq i64 %_3.i, 1
  %owned.i = getelementptr inbounds %"alloc::borrow::Cow<[u8]>", %"alloc::borrow::Cow<[u8]>"* %path, i64 0, i32 2
  %2 = getelementptr inbounds %"alloc::borrow::Cow<[u8]>", %"alloc::borrow::Cow<[u8]>"* %path, i64 0, i32 2, i64 1
  %3 = getelementptr inbounds %"alloc::borrow::Cow<[u8]>", %"alloc::borrow::Cow<[u8]>"* %path, i64 0, i32 2, i64 2
  %_2.sroa.0.0.in.i = bitcast [3 x i64]* %owned.i to [0 x i8]**
  %_2.sroa.0.0.i = load [0 x i8]*, [0 x i8]** %_2.sroa.0.0.in.i, align 8, !alias.scope !5129, !nonnull !1
  %.val.i = load i64, i64* %3, align 8, !alias.scope !5129
  %.val3.i = load i64, i64* %2, align 8, !alias.scope !5129
  %_2.sroa.5.0.i = select i1 %switch.i, i64 %.val.i, i64 %.val3.i
  %4 = icmp eq i64 %_2.sroa.5.0.i, 0
  br i1 %4, label %bb5, label %"_ZN70_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17h38574dfda84b4f2cE.exit"

"_ZN70_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17h38574dfda84b4f2cE.exit": ; preds = %start
  %5 = add i64 %_2.sroa.5.0.i, -1
  %6 = getelementptr inbounds [0 x i8], [0 x i8]* %_2.sroa.0.0.i, i64 0, i64 %5
  %.val.i.i = load i8, i8* %6, align 1, !alias.scope !5132
  %7 = icmp eq i8 %.val.i.i, 46
  br i1 %7, label %bb12, label %_ZN4bstr9ext_slice9ByteSlice10rfind_byte17h49aefeba57a625e8E.exit

bb5:                                              ; preds = %start
  %8 = getelementptr inbounds %"core::option::Option<alloc::borrow::Cow<[u8]>>", %"core::option::Option<alloc::borrow::Cow<[u8]>>"* %0, i64 0, i32 0, i64 0
  store i64 2, i64* %8, align 8
  br label %bb6

bb6:                                              ; preds = %bb5, %bb12, %bb25
  ret void

_ZN4bstr9ext_slice9ByteSlice10rfind_byte17h49aefeba57a625e8E.exit: ; preds = %"_ZN70_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17h38574dfda84b4f2cE.exit"
  %9 = load atomic i64, i64* bitcast (i8** getelementptr inbounds (%"core::sync::atomic::AtomicPtr<()>", %"core::sync::atomic::AtomicPtr<()>"* @_ZN6memchr3x867memrchr2FN17h5bc0baf729fbe004E, i64 0, i32 1) to i64*) monotonic, align 8, !noalias !5137
  %10 = inttoptr i64 %9 to { i64, i64 } (i8, [0 x i8]*, i64)*
  %11 = icmp ne i64 %9, 0
  tail call void @llvm.assume(i1 %11), !noalias !5137
  %12 = tail call { i64, i64 } %10(i8 47, [0 x i8]* noalias nonnull readonly align 1 %_2.sroa.0.0.i, i64 %_2.sroa.5.0.i)
  %.fca.0.extract.i.i = extractvalue { i64, i64 } %12, 0
  %.fca.1.extract.i.i = extractvalue { i64, i64 } %12, 1
  %switch.i41 = icmp eq i64 %.fca.0.extract.i.i, 0
  %13 = add i64 %.fca.1.extract.i.i, 1
  %..i = select i1 %switch.i41, i64 0, i64 %13
  br i1 %switch.i, label %bb17, label %bb19

bb12:                                             ; preds = %"_ZN70_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17h38574dfda84b4f2cE.exit"
  %14 = getelementptr inbounds %"core::option::Option<alloc::borrow::Cow<[u8]>>", %"core::option::Option<alloc::borrow::Cow<[u8]>>"* %0, i64 0, i32 0, i64 0
  store i64 2, i64* %14, align 8
  br label %bb6

bb17:                                             ; preds = %_ZN4bstr9ext_slice9ByteSlice10rfind_byte17h49aefeba57a625e8E.exit
  %15 = bitcast %"alloc::vec::Vec<u8>"* %path1 to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %15)
  %16 = bitcast %"alloc::vec::Vec<u8>"* %vector.i.i.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %16), !noalias !5146
  %_15.i.i.i.i.i.i = icmp eq i64 %.val.i, 0
  br i1 %_15.i.i.i.i.i.i, label %"_ZN5alloc3vec12Vec$LT$T$GT$13with_capacity17hc96a68e01ada40f4E.exit.i.i.i", label %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17h2d3f07a62eac3ef8E.exit.i.i.i.i.i.i"

"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17h2d3f07a62eac3ef8E.exit.i.i.i.i.i.i": ; preds = %bb17
  %17 = tail call i8* @__rust_alloc(i64 %.val.i, i64 1) #16, !noalias !5156
  %18 = icmp eq i8* %17, null
  br i1 %18, label %bb18.i.i.i.i.i.i, label %"_ZN5alloc3vec12Vec$LT$T$GT$13with_capacity17hc96a68e01ada40f4E.exit.i.i.i"

bb18.i.i.i.i.i.i:                                 ; preds = %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17h2d3f07a62eac3ef8E.exit.i.i.i.i.i.i"
; call alloc::alloc::handle_alloc_error
  tail call void @_ZN5alloc5alloc18handle_alloc_error17h31c2c94a892ecbd6E(i64 %.val.i, i64 1) #16, !noalias !5156
  unreachable

"_ZN5alloc3vec12Vec$LT$T$GT$13with_capacity17hc96a68e01ada40f4E.exit.i.i.i": ; preds = %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17h2d3f07a62eac3ef8E.exit.i.i.i.i.i.i", %bb17
  %ptr.0.i.i.i.i.i.i = phi i8* [ inttoptr (i64 1 to i8*), %bb17 ], [ %17, %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17h2d3f07a62eac3ef8E.exit.i.i.i.i.i.i" ]
  %19 = bitcast %"alloc::vec::Vec<u8>"* %vector.i.i.i to i8**
  store i8* %ptr.0.i.i.i.i.i.i, i8** %19, align 8, !alias.scope !5159, !noalias !5146
  %20 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %vector.i.i.i, i64 0, i32 1, i32 1
  store i64 %.val.i, i64* %20, align 8, !alias.scope !5159, !noalias !5146
  %21 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %vector.i.i.i, i64 0, i32 3
  store i64 0, i64* %21, align 8, !alias.scope !5159, !noalias !5146
; invoke alloc::vec::Vec<T>::reserve
  invoke fastcc void @"_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h8328aa9ad9c26b4cE"(%"alloc::vec::Vec<u8>"* nonnull align 8 dereferenceable(24) %vector.i.i.i, i64 %.val.i)
          to label %"_ZN63_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$5clone17ha8687e9b953bbe67E.exit" unwind label %funclet_bb5.i.i.i, !noalias !5146

funclet_bb5.i.i.i:                                ; preds = %"_ZN5alloc3vec12Vec$LT$T$GT$13with_capacity17hc96a68e01ada40f4E.exit.i.i.i"
  %cleanuppad.i.i.i = cleanuppad within none []
  call fastcc void bitcast (void (%"alloc::string::String"*)* @_ZN4core3ptr13drop_in_place17h16849cb7958e75d3E to void (%"alloc::vec::Vec<u8>"*)*)(%"alloc::vec::Vec<u8>"* nonnull %vector.i.i.i) #22 [ "funclet"(token %cleanuppad.i.i.i) ], !noalias !5146
  cleanupret from %cleanuppad.i.i.i unwind to caller

"_ZN63_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$5clone17ha8687e9b953bbe67E.exit": ; preds = %"_ZN5alloc3vec12Vec$LT$T$GT$13with_capacity17hc96a68e01ada40f4E.exit.i.i.i"
  %22 = getelementptr inbounds [0 x i8], [0 x i8]* %_2.sroa.0.0.i, i64 0, i64 0
  %self.idx.val.i.i.i.i.i = load i64, i64* %21, align 8, !noalias !5160
  %_13.i.i.i.i.i = add i64 %self.idx.val.i.i.i.i.i, %.val.i
  store i64 %_13.i.i.i.i.i, i64* %21, align 8, !noalias !5160
  %23 = bitcast %"alloc::vec::Vec<u8>"* %vector.i.i.i to [0 x i8]**
  %_3.idx.val.i1.i.i.i.i.i.i = load [0 x i8]*, [0 x i8]** %23, align 8, !noalias !5160, !nonnull !1
  %24 = getelementptr inbounds [0 x i8], [0 x i8]* %_3.idx.val.i1.i.i.i.i.i.i, i64 0, i64 %self.idx.val.i.i.i.i.i
  tail call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 1 %24, i8* nonnull align 1 %22, i64 %.val.i, i1 false) #16, !noalias !5163
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 %15, i8* nonnull align 8 %16, i64 24, i1 false), !noalias !5164
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %16), !noalias !5146
  %self.idx.i.i = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %path1, i64 0, i32 3
  %self.idx.val.i.i = load i64, i64* %self.idx.i.i, align 8, !noalias !5165
  %_26.i.i = icmp ult i64 %self.idx.val.i.i, %..i
  br i1 %_26.i.i, label %bb19.i.i, label %bb22

bb19.i.i:                                         ; preds = %"_ZN63_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$5clone17ha8687e9b953bbe67E.exit"
; invoke core::panicking::panic
  invoke void @_ZN4core9panicking5panic17h07bec01df8643458E([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [28 x i8] }>* @anon.aed952bceef054ab22098ec10f8b5504.48 to [0 x i8]*), i64 28, %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @anon.aed952bceef054ab22098ec10f8b5504.40 to %"core::panic::Location"*))
          to label %.noexc unwind label %funclet_bb23

.noexc:                                           ; preds = %bb19.i.i
  unreachable

bb19:                                             ; preds = %_ZN4bstr9ext_slice9ByteSlice10rfind_byte17h49aefeba57a625e8E.exit
  %_3.i.i.i = icmp ult i64 %.val3.i, %..i
  br i1 %_3.i.i.i, label %bb2.i.i.i, label %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h358af9de6dfcb22eE.exit"

bb2.i.i.i:                                        ; preds = %bb19
; call core::slice::slice_index_order_fail
  tail call void @_ZN4core5slice22slice_index_order_fail17h3334b3335c3bd3ecE(i64 %..i, i64 %.val3.i)
  unreachable

"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h358af9de6dfcb22eE.exit": ; preds = %bb19
  %25 = getelementptr inbounds [0 x i8], [0 x i8]* %_2.sroa.0.0.i, i64 0, i64 %..i
  %_8.i.i.i.i = sub i64 %.val3.i, %..i
  %26 = bitcast i8* %25 to [0 x i8]*
  br label %bb25

bb22:                                             ; preds = %"_ZN63_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$5clone17ha8687e9b953bbe67E.exit"
  store i64 0, i64* %self.idx.i.i, align 8, !noalias !5165
  %27 = bitcast %"alloc::vec::Vec<u8>"* %path1 to [0 x i8]**
  %_3.idx.val.i22.i.i = load [0 x i8]*, [0 x i8]** %27, align 8, !noalias !5165, !nonnull !1
  %_43.i.i = sub i64 %self.idx.val.i.i, %..i
  %_4.i.i.i.i.i = icmp eq i64 %_43.i.i, 0
  %28 = getelementptr inbounds [0 x i8], [0 x i8]* %_3.idx.val.i22.i.i, i64 0, i64 0
  br i1 %_4.i.i.i.i.i, label %bb24, label %bb2.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %bb22
  %_11.i.i.i.i.i = icmp eq i64 %..i, 0
  br i1 %_11.i.i.i.i.i, label %bb11.i.i.i.i.i, label %bb5.i.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %bb2.i.i.i.i.i
  %29 = getelementptr inbounds [0 x i8], [0 x i8]* %_3.idx.val.i22.i.i, i64 0, i64 %..i
  tail call void @llvm.memmove.p0i8.p0i8.i64(i8* nonnull align 1 %28, i8* nonnull align 1 %29, i64 %_43.i.i, i1 false) #16
  br label %bb11.i.i.i.i.i

bb11.i.i.i.i.i:                                   ; preds = %bb5.i.i.i.i.i, %bb2.i.i.i.i.i
  store i64 %_43.i.i, i64* %self.idx.i.i, align 8
  br label %bb24

bb24:                                             ; preds = %bb11.i.i.i.i.i, %bb22
  %_36.sroa.4.0..sroa_idx16 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %path1, i64 0, i32 1, i32 1
  %_36.sroa.4.0.copyload = load i64, i64* %_36.sroa.4.0..sroa_idx16, align 8
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %15)
  br label %bb25

bb25:                                             ; preds = %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h358af9de6dfcb22eE.exit", %bb24
  %_20.sroa.8.0 = phi i64 [ undef, %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h358af9de6dfcb22eE.exit" ], [ %_43.i.i, %bb24 ]
  %_20.sroa.7.0 = phi i64 [ %_8.i.i.i.i, %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h358af9de6dfcb22eE.exit" ], [ %_36.sroa.4.0.copyload, %bb24 ]
  %_20.sroa.5.0 = phi [0 x i8]* [ %26, %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h358af9de6dfcb22eE.exit" ], [ %_3.idx.val.i22.i.i, %bb24 ]
  %_20.sroa.0.0 = phi i64 [ 0, %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h358af9de6dfcb22eE.exit" ], [ 1, %bb24 ]
  %_20.sroa.0.0..sroa_idx = getelementptr inbounds %"core::option::Option<alloc::borrow::Cow<[u8]>>", %"core::option::Option<alloc::borrow::Cow<[u8]>>"* %0, i64 0, i32 0, i64 0
  store i64 %_20.sroa.0.0, i64* %_20.sroa.0.0..sroa_idx, align 8
  %_20.sroa.5.0..sroa_idx5 = getelementptr inbounds %"core::option::Option<alloc::borrow::Cow<[u8]>>", %"core::option::Option<alloc::borrow::Cow<[u8]>>"* %0, i64 0, i32 2
  %_20.sroa.5.0..sroa_cast = bitcast [3 x i64]* %_20.sroa.5.0..sroa_idx5 to [0 x i8]**
  store [0 x i8]* %_20.sroa.5.0, [0 x i8]** %_20.sroa.5.0..sroa_cast, align 8
  %30 = getelementptr inbounds %"core::option::Option<alloc::borrow::Cow<[u8]>>", %"core::option::Option<alloc::borrow::Cow<[u8]>>"* %0, i64 0, i32 2, i64 1
  store i64 %_20.sroa.7.0, i64* %30, align 8
  %31 = getelementptr inbounds %"core::option::Option<alloc::borrow::Cow<[u8]>>", %"core::option::Option<alloc::borrow::Cow<[u8]>>"* %0, i64 0, i32 2, i64 2
  store i64 %_20.sroa.8.0, i64* %31, align 8
  br label %bb6

funclet_bb23:                                     ; preds = %bb19.i.i
  %cleanuppad = cleanuppad within none []
  call fastcc void bitcast (void (%"alloc::string::String"*)* @_ZN4core3ptr13drop_in_place17h16849cb7958e75d3E to void (%"alloc::vec::Vec<u8>"*)*)(%"alloc::vec::Vec<u8>"* nonnull %path1) #22 [ "funclet"(token %cleanuppad) ]
  cleanupret from %cleanuppad unwind to caller
}
