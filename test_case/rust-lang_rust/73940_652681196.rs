llvm
; playground::save
; Function Attrs: naked noinline nounwind nonlazybind uwtable
define zeroext i1 @_ZN10playground4save17h7b21f7512f77098aE([8 x i64]* align 8 dereferenceable(64) %ctx) unnamed_addr #0 !dbg !6 {
start:
  %ctx.dbg.spill = alloca [8 x i64]*, align 8
  store [8 x i64]* %ctx, [8 x i64]** %ctx.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata [8 x i64]** %ctx.dbg.spill, metadata !18, metadata !DIExpression()), !dbg !19
  call void asm sideeffect alignstack inteldialect "pop    rax\0Amov    [rdi + 0x00], r12\0Amov    [rdi + 0x08], r13\0Amov    [rdi + 0x10], r14\0Amov    [rdi + 0x18], r15\0Amov    [rdi + 0x20], rbx\0Amov    [rdi + 0x28], rbp\0Amov    [rdi + 0x30], rsp\0Amov    [rdi + 0x38], rax\0Ajmp    rax", "{di},~{dirflag},~{fpsr},~{flags},~{memory}"([8 x i64]* %ctx), !dbg !20, !srcloc !21
  unreachable, !dbg !20
}
