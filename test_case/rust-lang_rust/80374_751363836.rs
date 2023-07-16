
; RUN: llc -mtriple=aarch64 -mattr=-neon < %s
define i128 @test(i128 %i) {
  %c = call i128 @llvm.ctpop(i128 %i)
  ret i128 %c
}

declare i128 @llvm.ctpop(i128)
