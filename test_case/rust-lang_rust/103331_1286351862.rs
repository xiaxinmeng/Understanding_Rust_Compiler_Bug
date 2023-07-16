
define i64 @result_nop_match_32(i64 %0) unnamed_addr #0 {
start:
  %_2 = and i64 %0, 4294967295
  %1 = icmp ne i64 %_2, 0
  %. = zext i1 %1 to i64
  %.sroa.4.0.extract.shift = and i64 %0, -4294967296
  %.sroa.04.0.insert.insert = or i64 %.sroa.4.0.extract.shift, %.
  ret i64 %.sroa.04.0.insert.insert
} 
