
RUN: llc < %s
define i8 @test(i128 %arg) {
  %vec = bitcast i128 %arg to <16 x i8>
  %red = tail call i8 @llvm.vector.reduce.mul.v16i8(<16 x i8> %vec)
  ret i8 %red
}

declare i8 @llvm.vector.reduce.mul.v16i8(<16 x i8>)
