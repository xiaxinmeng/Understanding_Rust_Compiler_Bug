
define void @test([70000 x i8]* %a) {
  %x = load volatile [70000 x i8], [70000 x i8]* %a
  ret void
}
