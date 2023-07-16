
entry-block:
  %src.sroa.0.0..sroa_idx = getelementptr inbounds [4096 x i8], [4096 x i8]* %1, i64 0, i64 0
  %src.sroa.0.0.copyload = load i8, i8* %src.sroa.0.0..sroa_idx, align 1
  ; repeat previous two lines 4095x

  %.fca.0.insert = insertvalue [4096 x i8] undef, i8 %src.sroa.0.0.copyload, 0
  %.fca.1.insert = insertvalue [4096 x i8] %.fca.0.insert, i8 %src.sroa.4.0.copyload, 1
  ; repeat previous line 4094x

  store volatile [4096 x i8] %.fca.4095.insert, [4096 x i8]* %0, align 1
  ret void
}