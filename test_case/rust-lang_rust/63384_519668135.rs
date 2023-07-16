
define { i8, i128 } @test(i128 %a) unnamed_addr {
entry:
  %tmp = lshr i128 %a, 64
  %tmp1 = and i128 %a, 18446744073709551615
  %tmp2 = mul nuw nsw i128 %tmp, 10
  %tmp3 = mul nuw nsw i128 %tmp1, 10
  %tmp4 = lshr i128 %tmp2, 64
  %tmp5 = trunc i128 %tmp4 to i8
  %tmp6 = mul i128 %tmp, 184467440737095516160
  %tmp7 = add i128 %tmp6, %tmp3
  %tmp8 = insertvalue { i8, i128 } undef, i8 %tmp5, 0
  %tmp9 = insertvalue { i8, i128 } %tmp8, i128 %tmp7, 1
  ret { i8, i128 } %tmp9
}
