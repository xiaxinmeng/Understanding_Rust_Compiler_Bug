
define void @test() {
  call void asm sideeffect inteldialect "call $0", "i"({} undef)
  ret void
}
