llvm
; std::hint::black_box(p.pointer);
define void @use_nonnull_a(i8** align 8 %p) unnamed_addr #1 {
  %0 = bitcast i8** %p to {}**
  %_3 = load {}*, {}** %0, align 8
  %_2 = call {}* @core_hint_black_box({}* %_3)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; std::hint::black_box({*p}.pointer);
define void @use_nonnull_b(i8** align 8 %p) unnamed_addr #1 {
  %p1 = load i8*, i8** %p, align 8, !nonnull !3, !noundef !3
  %_4 = bitcast i8* %p1 to {}*
  %_3 = call {}* @core_hint_black_box({}* %_4)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}
