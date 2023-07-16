 llvm
define <2 x i64> @internal(<2 x i64>, <2 x i64>) unnamed_addr #0 {
  %3 = bitcast <2 x i64> %0 to i128
  %4 = bitcast <2 x i64> %1 to i128
  %5 = mul i128 %4, %3
  %6 = bitcast i128 %5 to <2 x i64>
  ret <2 x i64> %6
}
