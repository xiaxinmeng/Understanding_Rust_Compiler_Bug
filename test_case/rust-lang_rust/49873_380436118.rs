ll
; encoding_rs::Encoding::ascii_valid_up_to
; Function Attrs: readonly uwtable
define i32 @_ZN11encoding_rs8Encoding17ascii_valid_up_to17h3e9b04dc68770813E([0 x i8]* noalias nonnull readonly %bytes.0, i32 %bytes.1) unnamed_addr #6 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = icmp ugt i32 %bytes.1, 15
  br i1 %0, label %bb3.i.i, label %bb30.preheader.i.i

bb3.i.i:                                          ; preds = %start
  %1 = add i32 %bytes.1, -16
  %2 = ptrtoint [0 x i8]* %bytes.0 to i32
  %3 = and i32 %2, 15
  %4 = icmp eq i32 %3, 0
  br i1 %4, label %bb5.i.i.preheader, label %bb18.i.i.preheader

bb18.i.i.preheader:                               ; preds = %bb3.i.i
  br label %bb18.i.i

bb5.i.i.preheader:                                ; preds = %bb3.i.i
  br label %bb5.i.i

bb5.i.i:                                          ; preds = %bb5.i.i.preheader, %bb10.i.i
  %offset.0.i.i = phi i32 [ %10, %bb10.i.i ], [ 0, %bb5.i.i.preheader ]
  %5 = getelementptr inbounds [0 x i8], [0 x i8]* %bytes.0, i32 0, i32 %offset.0.i.i
  %6 = bitcast i8* %5 to <16 x i8>*
  %7 = load <16 x i8>, <16 x i8>* %6, align 16, !alias.scope !6596, !noalias !6601
  %8 = tail call i32 @llvm.x86.sse2.pmovmskb.128(<16 x i8> %7) #7
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %bb10.i.i, label %bb15.sink.split.i.i.loopexit

bb10.i.i:                                         ; preds = %bb5.i.i
  %10 = add i32 %offset.0.i.i, 16
  %11 = icmp ugt i32 %10, %1
  br i1 %11, label %bb30.preheader.i.i.loopexit, label %bb5.i.i

bb15.sink.split.i.i.loopexit:                     ; preds = %bb5.i.i
  br label %bb15.sink.split.i.i

bb15.sink.split.i.i.loopexit37:                   ; preds = %bb18.i.i
  br label %bb15.sink.split.i.i

bb15.sink.split.i.i:                              ; preds = %bb15.sink.split.i.i.loopexit37, %bb15.sink.split.i.i.loopexit
  %.sink35.i.i = phi i32 [ %8, %bb15.sink.split.i.i.loopexit ], [ %15, %bb15.sink.split.i.i.loopexit37 ]
  %offset.1.sink.i.i = phi i32 [ %offset.0.i.i, %bb15.sink.split.i.i.loopexit ], [ %offset.1.i.i, %bb15.sink.split.i.i.loopexit37 ]
  %12 = tail call i32 @llvm.cttz.i32(i32 %.sink35.i.i, i1 false) #7
  %13 = add i32 %offset.1.sink.i.i, %12
  br label %_ZN11encoding_rs5ascii17ascii_valid_up_to17h5c1d65b6f83b80ecE.exit

bb18.i.i:                                         ; preds = %bb18.i.i.preheader, %bb23.i.i
  %offset.1.i.i = phi i32 [ %17, %bb23.i.i ], [ 0, %bb18.i.i.preheader ]
  %14 = getelementptr inbounds [0 x i8], [0 x i8]* %bytes.0, i32 0, i32 %offset.1.i.i
  %simd.0.ptr.sroa_cast.i.i.i = bitcast i8* %14 to <16 x i8>*
  %simd.0.copyload.i.i.i = load <16 x i8>, <16 x i8>* %simd.0.ptr.sroa_cast.i.i.i, align 1, !alias.scope !6596, !noalias !6601
  %15 = tail call i32 @llvm.x86.sse2.pmovmskb.128(<16 x i8> %simd.0.copyload.i.i.i) #7
  %16 = icmp eq i32 %15, 0
  br i1 %16, label %bb23.i.i, label %bb15.sink.split.i.i.loopexit37

bb23.i.i:                                         ; preds = %bb18.i.i
  %17 = add i32 %offset.1.i.i, 16
  %18 = icmp ugt i32 %17, %1
  br i1 %18, label %bb30.preheader.i.i.loopexit38, label %bb18.i.i

bb30.preheader.i.i.loopexit:                      ; preds = %bb10.i.i
  br label %bb30.preheader.i.i

bb30.preheader.i.i.loopexit38:                    ; preds = %bb23.i.i
  br label %bb30.preheader.i.i

bb30.preheader.i.i:                               ; preds = %bb30.preheader.i.i.loopexit38, %bb30.preheader.i.i.loopexit, %start
  %offset.2.ph.i.i = phi i32 [ 0, %start ], [ %10, %bb30.preheader.i.i.loopexit ], [ %17, %bb30.preheader.i.i.loopexit38 ]
  %19 = icmp ult i32 %offset.2.ph.i.i, %bytes.1
  br i1 %19, label %bb33.i.i.preheader, label %_ZN11encoding_rs5ascii17ascii_valid_up_to17h5c1d65b6f83b80ecE.exit

bb33.i.i.preheader:                               ; preds = %bb30.preheader.i.i
  br label %bb33.i.i

bb33.i.i:                                         ; preds = %bb33.i.i.preheader, %bb35.i.i
  %offset.247.i.i = phi i32 [ %23, %bb35.i.i ], [ %offset.2.ph.i.i, %bb33.i.i.preheader ]
  %20 = getelementptr inbounds [0 x i8], [0 x i8]* %bytes.0, i32 0, i32 %offset.247.i.i
  %21 = load i8, i8* %20, align 1, !alias.scope !6596, !noalias !6601
  %22 = icmp slt i8 %21, 0
  br i1 %22, label %_ZN11encoding_rs5ascii17ascii_valid_up_to17h5c1d65b6f83b80ecE.exit.loopexit, label %bb35.i.i

bb35.i.i:                                         ; preds = %bb33.i.i
  %23 = add nuw i32 %offset.247.i.i, 1
  %24 = icmp ult i32 %23, %bytes.1
  br i1 %24, label %bb33.i.i, label %_ZN11encoding_rs5ascii17ascii_valid_up_to17h5c1d65b6f83b80ecE.exit.loopexit

_ZN11encoding_rs5ascii17ascii_valid_up_to17h5c1d65b6f83b80ecE.exit.loopexit: ; preds = %bb35.i.i, %bb33.i.i
  %_0.0.i.ph = phi i32 [ %bytes.1, %bb35.i.i ], [ %offset.247.i.i, %bb33.i.i ]
  br label %_ZN11encoding_rs5ascii17ascii_valid_up_to17h5c1d65b6f83b80ecE.exit

_ZN11encoding_rs5ascii17ascii_valid_up_to17h5c1d65b6f83b80ecE.exit: ; preds = %_ZN11encoding_rs5ascii17ascii_valid_up_to17h5c1d65b6f83b80ecE.exit.loopexit, %bb15.sink.split.i.i, %bb30.preheader.i.i
  %_0.0.i = phi i32 [ %bytes.1, %bb30.preheader.i.i ], [ %13, %bb15.sink.split.i.i ], [ %_0.0.i.ph, %_ZN11encoding_rs5ascii17ascii_valid_up_to17h5c1d65b6f83b80ecE.exit.loopexit ]
  ret i32 %_0.0.i
}
