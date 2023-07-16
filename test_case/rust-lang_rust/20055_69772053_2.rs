
  %x.sroa.0.0..sroa_cast1.i = bitcast i8* %0 to i24*
  store i24 197121, i24* %x.sroa.0.0..sroa_cast1.i, align 1
  tail call void @je_sdallocx(i8* %0, i64 3, i32 0)
  tail call void @je_sdallocx(i8* undef, i64 1, i32 0)
  ret void
