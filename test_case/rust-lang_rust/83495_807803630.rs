llvm
define dso_local void @main(i64* %arg) unnamed_addr {
  %a = tail call { i64, i8 } asm "", "={bx},={ah}"()
  %_4.0 = extractvalue { i64, i8 } %a, 0
  %_4.1 = extractvalue { i64, i8 } %a, 1
  store i64 %_4.0, i64* %arg
  %1 = xor i8 %_4.1, 0
  call void @banana()
  %_30.not = icmp eq i8 %1, 64
  br i1 %_30.not, label %bb7, label %bb6

bb6:
  unreachable

bb7:
  unreachable
}

declare dso_local void @banana() unnamed_addr
