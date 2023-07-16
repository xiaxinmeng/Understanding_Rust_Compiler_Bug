
define i1 @test(i32 %x) {
  %y = lshr i32 %x, 1
  %z = sub i32 %x, %y
  %r = icmp ult i32 %z, %y
  ret i1 %r
}
