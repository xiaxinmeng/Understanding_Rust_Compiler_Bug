
  %_2.sroa.4.0.iter.sroa_idx = getelementptr inbounds i8, ptr %iter, i64 48
  store i64 0, ptr %_2.sroa.4.0.iter.sroa_idx, align 8
  %_2.sroa.5.0.iter.sroa_idx = getelementptr inbounds i8, ptr %iter, i64 56
  store i64 6, ptr %_2.sroa.5.0.iter.sroa_idx, align 8
  %self1.i = getelementptr inbounds %"core::array::iter::IntoIter<usize, 6>", ptr %iter, i64 0, i32 1
  %_4.i.i = getelementptr inbounds %"core::array::iter::IntoIter<usize, 6>", ptr %iter, i64 0, i32 1, i32 1
  %_4.val.i.i = load i64, ptr %_4.i.i, align 8, !alias.scope !2
  %0 = load i64, ptr %self1.i, align 8, !alias.scope !2
