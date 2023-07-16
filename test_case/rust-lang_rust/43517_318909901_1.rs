
; Function Attrs: norecurse nounwind readonly uwtable
define i16 @"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$3all_28_$u7b$$u7b$closure$u7d$$u7d$17h5b298b7604579d38E"(%closure* nocapture readnone, i32* noalias nocapture readonly dereferenceable(4)) unnamed_addr #0 {
start:
  %_0.sroa.0 = alloca i8, align 1
  %_0.sroa.3 = alloca i8, align 1
  %2 = load i32, i32* %1, align 4
  %3 = icmp eq i32 %2, 1
  br i1 %3, label %bb3, label %bb4

bb3:                                              ; preds = %start
  store i8 1, i8* %_0.sroa.0, align 1
  br label %bb4

bb4:                                              ; preds = %start, %bb3
  %.sink = phi i8* [ %_0.sroa.3, %bb3 ], [ %_0.sroa.0, %start ]
  store i8 0, i8* %.sink, align 1
  %_0.sroa.0.0._0.sroa.0.0._0.sroa.0.0. = load i8, i8* %_0.sroa.0, align 1
  %_0.sroa.3.0._0.sroa.3.0._0.sroa.3.0. = load i8, i8* %_0.sroa.3, align 1
  %_0.sroa.3.0.insert.ext = zext i8 %_0.sroa.3.0._0.sroa.3.0._0.sroa.3.0. to i16
  %_0.sroa.3.0.insert.shift = shl nuw i16 %_0.sroa.3.0.insert.ext, 8
  %_0.sroa.0.0.insert.ext = zext i8 %_0.sroa.0.0._0.sroa.0.0._0.sroa.0.0. to i16
  %_0.sroa.0.0.insert.insert = or i16 %_0.sroa.3.0.insert.shift, %_0.sroa.0.0.insert.ext
  ret i16 %_0.sroa.0.0.insert.insert
}
