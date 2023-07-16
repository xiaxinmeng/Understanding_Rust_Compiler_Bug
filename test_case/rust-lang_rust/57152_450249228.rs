
define void @test(i8 %byte) {
  %t = alloca { i8, i8 }, align 1
  %x4 = and i8 %byte, 1
  %x5 = icmp eq i8 %x4, 1
  %x6 = and i8 %byte, 2
  %x7 = icmp eq i8 %x6, 2
  %x8 = bitcast { i8, i8 }* %t to i8*
  %x9 = zext i1 %x5 to i8
  store i8 %x9, i8* %x8, align 1
  %x10 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %t, i32 0, i32 1
  %x11 = zext i1 %x7 to i8
  store i8 %x11, i8* %x10, align 1
  ret void
}
