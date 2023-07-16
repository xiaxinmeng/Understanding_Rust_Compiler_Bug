asm
 fn from_elem_u16_std(n: u16) -> Vec<u16> {
 push    rbx
 mov     rbx, rdi
     unsafe { __rust_alloc(layout.size(), layout.align()) } (liballoc/alloc.rs:80)
     mov     edi, 200
     mov     esi, 2
     call    qword, ptr, [rip, +, __rust_alloc@GOTPCREL]
     Ok(memory) => memory, (liballoc/raw_vec.rs:184)
     test    rax, rax
     je      .LBB7_1
     movabs  rcx, 723401728380766730
     unsafe { intrinsics::move_val_init(&mut *dst, src) } (libcore/ptr/mod.rs:901)
     mov     qword, ptr, [rax, +, 192], rcx
     movaps  xmm0, xmmword, ptr, [rip, +, .LCPI7_0]
     movups  xmmword, ptr, [rax, +, 176], xmm0
     movups  xmmword, ptr, [rax, +, 160], xmm0
     movups  xmmword, ptr, [rax, +, 144], xmm0
     movups  xmmword, ptr, [rax, +, 128], xmm0
     movups  xmmword, ptr, [rax, +, 112], xmm0
     movups  xmmword, ptr, [rax, +, 96], xmm0
     movups  xmmword, ptr, [rax, +, 80], xmm0
     movups  xmmword, ptr, [rax, +, 64], xmm0
     movups  xmmword, ptr, [rax, +, 48], xmm0
     movups  xmmword, ptr, [rax, +, 32], xmm0
     movups  xmmword, ptr, [rax, +, 16], xmm0
     movups  xmmword, ptr, [rax], xmm0
     v (liballoc/vec.rs:1842)
     mov     qword, ptr, [rbx], rax
     movaps  xmm0, xmmword, ptr, [rip, +, .LCPI7_1]
     movups  xmmword, ptr, [rbx, +, 8], xmm0
 }
 pop     rbx
 ret
.LBB7_1:
     Err(_) => handle_alloc_error(layout), (liballoc/raw_vec.rs:185)
     mov     edi, 200
     mov     esi, 2
     call    qword, ptr, [rip, +, _ZN5alloc5alloc18handle_alloc_error17h7f6a7a00afe6ad92E@GOTPCREL]
     ud2
