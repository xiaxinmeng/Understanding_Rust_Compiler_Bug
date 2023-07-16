llvm
define internal void @banana() unnamed_addr {
start:
  tail call void asm sideeffect "mov $0, %rax", "i,~{dirflag},~{fpsr},~{flags}"({} undef)
  ret void
}
