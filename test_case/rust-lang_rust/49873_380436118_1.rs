ll
; encoding_rs::utf_8::utf8_valid_up_to
; Function Attrs: uwtable
define internal fastcc i32 @_ZN11encoding_rs5utf_816utf8_valid_up_to17he6ec0248b61d42c5E([0 x i8]* noalias nonnull readonly %bytes.0, i32 %bytes.1) unnamed_addr #4 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  br label %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17he71b553e81d1e1aeE.exit.i"

bb1.i.i.i.i:                                      ; preds = %bb73.i
; call core::slice::slice_index_order_fail
  tail call void @_ZN4core5slice22slice_index_order_fail17hd587cac96d801a96E(i32 %94, i32 %bytes.1)
  unreachable

"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17he71b553e81d1e1aeE.exit.i": ; preds = %bb73.i, %start
  %index.0151.i = phi i32 [ 0, %start ], [ %94, %bb73.i ]
  %0 = getelementptr inbounds [0 x i8], [0 x i8]* %bytes.0, i32 0, i32 %index.0151.i
  %1 = sub i32 %bytes.1, %index.0151.i
  %2 = icmp ugt i32 %1, 15
  br i1 %2, label %bb3.i.i, label %bb29.i.i

bb3.i.i:                                          ; preds = %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17he71b553e81d1e1aeE.exit.i"
  %3 = add i32 %1, -16
  %4 = ptrtoint i8* %0 to i32
  %5 = and i32 %4, 15
  %6 = icmp eq i32 %5, 0
  %7 = bitcast i8* %0 to <16 x i8>*
  br i1 %6, label %bb5.preheader.i.i, label %bb18.preheader.i.i

bb18.preheader.i.i:                               ; preds = %bb3.i.i
  %simd.0.copyload.i46.i.i = load <16 x i8>, <16 x i8>* %7, align 1, !alias.scope !27, !noalias !32
  %8 = tail call i32 @llvm.x86.sse2.pmovmskb.128(<16 x i8> %simd.0.copyload.i46.i.i) #10
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %bb23.i.i.preheader, label %bb22.i.i

bb23.i.i.preheader:                               ; preds = %bb18.preheader.i.i
  br label %bb23.i.i

bb5.preheader.i.i:                                ; preds = %bb3.i.i
  %10 = load <16 x i8>, <16 x i8>* %7, align 16, !alias.scope !27, !noalias !37
  %11 = tail call i32 @llvm.x86.sse2.pmovmskb.128(<16 x i8> %10) #10
  %12 = icmp eq i32 %11, 0
  br i1 %12, label %bb10.i.i.preheader, label %bb9.i.i

bb10.i.i.preheader:                               ; preds = %bb5.preheader.i.i
  br label %bb10.i.i

bb5.i.i:                                          ; preds = %bb10.i.i
  %13 = getelementptr inbounds i8, i8* %0, i32 %20
  %14 = bitcast i8* %13 to <16 x i8>*
  %15 = load <16 x i8>, <16 x i8>* %14, align 16, !alias.scope !27, !noalias !37
  %16 = tail call i32 @llvm.x86.sse2.pmovmskb.128(<16 x i8> %15) #10
  %17 = icmp eq i32 %16, 0
  br i1 %17, label %bb10.i.i, label %bb9.i.i

bb9.i.i:                                          ; preds = %bb5.i.i, %bb5.preheader.i.i
  %offset.0.lcssa.i.i = phi i32 [ 0, %bb5.preheader.i.i ], [ %20, %bb5.i.i ]
  %.lcssa34.i.i = phi i32 [ %11, %bb5.preheader.i.i ], [ %16, %bb5.i.i ]
  %18 = tail call i32 @llvm.cttz.i32(i32 %.lcssa34.i.i, i1 false) #10, !range !40
  %19 = add i32 %18, %offset.0.lcssa.i.i
  br label %bb7.i.sink.split

bb10.i.i:                                         ; preds = %bb10.i.i.preheader, %bb5.i.i
  %offset.043.i.i = phi i32 [ %20, %bb5.i.i ], [ 0, %bb10.i.i.preheader ]
  %20 = add i32 %offset.043.i.i, 16
  %21 = icmp ugt i32 %20, %3
  br i1 %21, label %bb29.i.i, label %bb5.i.i

bb18.i.i:                                         ; preds = %bb23.i.i
  %22 = getelementptr inbounds i8, i8* %0, i32 %27
  %simd.0.ptr.sroa_cast.i.i.i = bitcast i8* %22 to <16 x i8>*
  %simd.0.copyload.i.i.i = load <16 x i8>, <16 x i8>* %simd.0.ptr.sroa_cast.i.i.i, align 1, !alias.scope !27, !noalias !32
  %23 = tail call i32 @llvm.x86.sse2.pmovmskb.128(<16 x i8> %simd.0.copyload.i.i.i) #10
  %24 = icmp eq i32 %23, 0
  br i1 %24, label %bb23.i.i, label %bb22.i.i

bb22.i.i:                                         ; preds = %bb18.i.i, %bb18.preheader.i.i
  %offset.1.lcssa.i.i = phi i32 [ 0, %bb18.preheader.i.i ], [ %27, %bb18.i.i ]
  %.lcssa38.i.i = phi i32 [ %8, %bb18.preheader.i.i ], [ %23, %bb18.i.i ]
  %25 = tail call i32 @llvm.cttz.i32(i32 %.lcssa38.i.i, i1 false) #10, !range !40
  %26 = add i32 %25, %offset.1.lcssa.i.i
  br label %bb7.i.sink.split

bb23.i.i:                                         ; preds = %bb23.i.i.preheader, %bb18.i.i
  %offset.147.i.i = phi i32 [ %27, %bb18.i.i ], [ 0, %bb23.i.i.preheader ]
  %27 = add i32 %offset.147.i.i, 16
  %28 = icmp ugt i32 %27, %3
  br i1 %28, label %bb29.i.i, label %bb18.i.i

bb29.i.i:                                         ; preds = %bb23.i.i, %bb10.i.i, %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17he71b553e81d1e1aeE.exit.i"
  %offset.2.i.i = phi i32 [ 0, %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17he71b553e81d1e1aeE.exit.i" ], [ %20, %bb10.i.i ], [ %27, %bb23.i.i ]
  %29 = icmp ult i32 %offset.2.i.i, %1
  br i1 %29, label %bb33.i.i.preheader, label %bb5

bb33.i.i.preheader:                               ; preds = %bb29.i.i
  br label %bb33.i.i

bb33.i.i:                                         ; preds = %bb33.i.i.preheader, %bb35.i.i
  %offset.342.i.i = phi i32 [ %33, %bb35.i.i ], [ %offset.2.i.i, %bb33.i.i.preheader ]
  %30 = getelementptr inbounds i8, i8* %0, i32 %offset.342.i.i
  %31 = load i8, i8* %30, align 1, !alias.scope !27, !noalias !41
  %32 = icmp slt i8 %31, 0
  br i1 %32, label %bb7.i, label %bb35.i.i

bb35.i.i:                                         ; preds = %bb33.i.i
  %33 = add nuw i32 %offset.342.i.i, 1
  %34 = icmp ult i32 %33, %1
  br i1 %34, label %bb33.i.i, label %bb5

bb7.i.sink.split:                                 ; preds = %bb9.i.i, %bb22.i.i
  %.sink66 = phi i32 [ %26, %bb22.i.i ], [ %19, %bb9.i.i ]
  %35 = getelementptr inbounds i8, i8* %0, i32 %.sink66
  %36 = load i8, i8* %35, align 1, !alias.scope !27, !noalias !41
  br label %bb7.i

bb7.i:                                            ; preds = %bb33.i.i, %bb7.i.sink.split
  %_12.sroa.1294.1.ph.i = phi i32 [ %.sink66, %bb7.i.sink.split ], [ %offset.342.i.i, %bb33.i.i ]
  %_12.sroa.8.1.ph.i = phi i8 [ %36, %bb7.i.sink.split ], [ %31, %bb33.i.i ]
  %37 = add i32 %_12.sroa.1294.1.ph.i, %index.0151.i
  br label %bb9.i

bb9.i:                                            ; preds = %bb72.i, %bb7.i
  %first.0.i = phi i8 [ %_12.sroa.8.1.ph.i, %bb7.i ], [ %92, %bb72.i ]
  %index.1.i = phi i32 [ %37, %bb7.i ], [ %47, %bb72.i ]
  %38 = zext i8 %first.0.i to i32
  %39 = getelementptr inbounds [256 x i8], [256 x i8]* @_ZN11encoding_rs10utf_8_core15UTF8_CHAR_WIDTH17hf8319a77b9cec8e4E, i32 0, i32 %38
  %40 = load i8, i8* %39, align 1
  switch i8 %40, label %bb5 [
    i8 2, label %bb11.i
    i8 3, label %bb12.i
    i8 4, label %bb13.i
  ]

bb11.i:                                           ; preds = %bb9.i
  %41 = add i32 %index.1.i, 1
  %42 = icmp ult i32 %41, %bytes.1
  br i1 %42, label %bb20.i, label %bb5

bb12.i:                                           ; preds = %bb9.i
  %43 = add i32 %index.1.i, 1
  %44 = icmp ult i32 %43, %bytes.1
  br i1 %44, label %bb25.i, label %bb5

bb13.i:                                           ; preds = %bb9.i
  %45 = add i32 %index.1.i, 1
  %46 = icmp ult i32 %45, %bytes.1
  br i1 %46, label %bb48.i, label %bb5

bb15.i:                                           ; preds = %bb67.i, %bb43.i, %bb20.i
  %index.2.i = phi i32 [ %84, %bb67.i ], [ %59, %bb43.i ], [ %41, %bb20.i ]
  %47 = add i32 %index.2.i, 1
  %48 = icmp eq i32 %47, %bytes.1
  br i1 %48, label %bb5, label %bb71.i

bb20.i:                                           ; preds = %bb11.i
  %49 = getelementptr inbounds [0 x i8], [0 x i8]* %bytes.0, i32 0, i32 %41
  %50 = load i8, i8* %49, align 1, !alias.scope !42, !noalias !43
  %51 = and i8 %50, -64
  %52 = icmp eq i8 %51, -128
  br i1 %52, label %bb15.i, label %bb5

bb25.i:                                           ; preds = %bb12.i
  %53 = getelementptr inbounds [0 x i8], [0 x i8]* %bytes.0, i32 0, i32 %43
  %54 = load i8, i8* %53, align 1, !alias.scope !42, !noalias !43
  %cond9.i = icmp eq i8 %first.0.i, -32
  %55 = icmp ult i8 %54, -64
  %56 = and i8 %54, -32
  %57 = icmp eq i8 %56, -96
  %58 = and i1 %cond9.i, %57
  br i1 %58, label %bb26.i, label %bb30.i

bb26.i:                                           ; preds = %bb37.i, %bb34.i, %bb30.i, %bb25.i
  %59 = add i32 %index.1.i, 2
  %60 = icmp ult i32 %59, %bytes.1
  br i1 %60, label %bb43.i, label %bb5

bb30.i:                                           ; preds = %bb25.i
  %first.0.off101.i = add i8 %first.0.i, 31
  %61 = icmp ult i8 %first.0.off101.i, 12
  %62 = icmp slt i8 %54, 0
  %or.cond76.i = and i1 %61, %62
  %or.cond77.i = and i1 %55, %or.cond76.i
  br i1 %or.cond77.i, label %bb26.i, label %bb34.i

bb34.i:                                           ; preds = %bb30.i
  %cond10.i = icmp eq i8 %first.0.i, -19
  %or.cond78.i = and i1 %cond10.i, %62
  %63 = icmp ult i8 %54, -96
  %or.cond79.i = and i1 %63, %or.cond78.i
  br i1 %or.cond79.i, label %bb26.i, label %bb37.i

bb37.i:                                           ; preds = %bb34.i
  %64 = and i8 %first.0.i, -2
  %65 = icmp eq i8 %64, -18
  %or.cond81.i = and i1 %65, %62
  %or.cond82.i = and i1 %55, %or.cond81.i
  br i1 %or.cond82.i, label %bb26.i, label %bb5

bb43.i:                                           ; preds = %bb26.i
  %66 = getelementptr inbounds [0 x i8], [0 x i8]* %bytes.0, i32 0, i32 %59
  %67 = load i8, i8* %66, align 1, !alias.scope !42, !noalias !43
  %68 = and i8 %67, -64
  %69 = icmp eq i8 %68, -128
  br i1 %69, label %bb15.i, label %bb5

bb48.i:                                           ; preds = %bb13.i
  %70 = getelementptr inbounds [0 x i8], [0 x i8]* %bytes.0, i32 0, i32 %45
  %71 = load i8, i8* %70, align 1, !alias.scope !42, !noalias !43
  %cond.i = icmp eq i8 %first.0.i, -16
  %.off.i = add i8 %71, 112
  %72 = icmp ult i8 %.off.i, 48
  %73 = and i1 %cond.i, %72
  br i1 %73, label %bb49.i, label %bb53.i

bb49.i:                                           ; preds = %bb57.i, %bb53.i, %bb48.i
  %74 = add i32 %index.1.i, 2
  %75 = icmp ult i32 %74, %bytes.1
  br i1 %75, label %bb62.i, label %bb5

bb53.i:                                           ; preds = %bb48.i
  %76 = icmp ult i8 %71, -64
  %first.0.off.i = add i8 %first.0.i, 15
  %77 = icmp ult i8 %first.0.off.i, 3
  %78 = icmp slt i8 %71, 0
  %or.cond86.i = and i1 %77, %78
  %or.cond87.i = and i1 %76, %or.cond86.i
  br i1 %or.cond87.i, label %bb49.i, label %bb57.i

bb57.i:                                           ; preds = %bb53.i
  %cond8.i = icmp eq i8 %first.0.i, -12
  %or.cond88.i = and i1 %cond8.i, %78
  %79 = icmp ult i8 %71, -112
  %or.cond89.i = and i1 %79, %or.cond88.i
  br i1 %or.cond89.i, label %bb49.i, label %bb5

bb62.i:                                           ; preds = %bb49.i
  %80 = getelementptr inbounds [0 x i8], [0 x i8]* %bytes.0, i32 0, i32 %74
  %81 = load i8, i8* %80, align 1, !alias.scope !42, !noalias !43
  %82 = and i8 %81, -64
  %83 = icmp eq i8 %82, -128
  br i1 %83, label %bb64.i, label %bb5

bb64.i:                                           ; preds = %bb62.i
  %84 = add i32 %index.1.i, 3
  %85 = icmp ult i32 %84, %bytes.1
  br i1 %85, label %bb67.i, label %bb5

bb67.i:                                           ; preds = %bb64.i
  %86 = getelementptr inbounds [0 x i8], [0 x i8]* %bytes.0, i32 0, i32 %84
  %87 = load i8, i8* %86, align 1, !alias.scope !42, !noalias !43
  %88 = and i8 %87, -64
  %89 = icmp eq i8 %88, -128
  br i1 %89, label %bb15.i, label %bb5

bb71.i:                                           ; preds = %bb15.i
  %90 = icmp ult i32 %47, %bytes.1
  br i1 %90, label %bb72.i, label %panic7.i, !prof !44

bb72.i:                                           ; preds = %bb71.i
  %91 = getelementptr inbounds [0 x i8], [0 x i8]* %bytes.0, i32 0, i32 %47
  %92 = load i8, i8* %91, align 1, !alias.scope !42, !noalias !43
  %93 = icmp sgt i8 %92, -1
  br i1 %93, label %bb73.i, label %bb9.i

bb73.i:                                           ; preds = %bb72.i
  %94 = add i32 %index.2.i, 2
  %95 = icmp ugt i32 %94, %bytes.1
  br i1 %95, label %bb1.i.i.i.i, label %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17he71b553e81d1e1aeE.exit.i"

panic7.i:                                         ; preds = %bb71.i
; call core::panicking::panic_bounds_check
  tail call void @_ZN4core9panicking18panic_bounds_check17h24c830ebecd41baaE({ [0 x i32], { [0 x i8]*, i32 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly dereferenceable(16) bitcast ({ { [0 x i8]*, i32 }, i32, i32 }* @panic_bounds_check_loc.C to { [0 x i32], { [0 x i8]*, i32 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*), i32 %47, i32 %bytes.1)
  unreachable

bb5:                                              ; preds = %bb29.i.i, %bb35.i.i, %bb67.i, %bb64.i, %bb62.i, %bb49.i, %bb57.i, %bb13.i, %bb43.i, %bb26.i, %bb37.i, %bb12.i, %bb20.i, %bb11.i, %bb9.i, %bb15.i
  %_0.0 = phi i32 [ %37, %bb67.i ], [ %37, %bb64.i ], [ %37, %bb62.i ], [ %37, %bb49.i ], [ %37, %bb57.i ], [ %37, %bb13.i ], [ %37, %bb43.i ], [ %37, %bb26.i ], [ %37, %bb37.i ], [ %37, %bb12.i ], [ %37, %bb20.i ], [ %37, %bb11.i ], [ %37, %bb9.i ], [ %bytes.1, %bb15.i ], [ %bytes.1, %bb35.i.i ], [ %bytes.1, %bb29.i.i ]
  ret i32 %_0.0
}
