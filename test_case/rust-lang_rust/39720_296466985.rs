
define i64 @main(i64, i8**) unnamed_addr {
start:

  %veci8 = alloca <3 x i8>
  %veci16 = alloca <3 x i16>

  %2 = load <3 x i8>, <3 x i8>* %veci8
  %3 = sext <3 x i8> %2 to <3 x i16>
  store <3 x i16> %3, <3 x i16>* %veci16

  ret i64 0
}
