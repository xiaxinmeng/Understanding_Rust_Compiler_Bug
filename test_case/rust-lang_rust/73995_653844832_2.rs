asm
 fn from_elem_u8_std(n: u8) -> Vec<u8> {
 push    rbx
 mov     rbx, rdi
     unsafe { __rust_alloc(layout.size(), layout.align()) } (liballoc/alloc.rs:80)
     mov     edi, 100
     mov     esi, 1
     call    qword, ptr, [rip, +, __rust_alloc@GOTPCREL]
     Ok(memory) => memory, (liballoc/raw_vec.rs:184)
     test    rax, rax
     je      .LBB5_1
     unsafe { write_bytes(dst, val, count) } (libcore/intrinsics.rs:2291)
     movaps  xmm0, xmmword, ptr, [rip, +, .LCPI5_0]
     movups  xmmword, ptr, [rax, +, 80], xmm0
     movups  xmmword, ptr, [rax, +, 64], xmm0
     movups  xmmword, ptr, [rax, +, 48], xmm0
     movups  xmmword, ptr, [rax, +, 32], xmm0
     movups  xmmword, ptr, [rax, +, 16], xmm0
     movups  xmmword, ptr, [rax], xmm0
     mov     dword, ptr, [rax, +, 96], 168430090
     v (liballoc/vec.rs:1829)
     mov     qword, ptr, [rbx], rax
     movaps  xmm0, xmmword, ptr, [rip, +, .LCPI5_1]
     movups  xmmword, ptr, [rbx, +, 8], xmm0
 }
 pop     rbx
 ret
.LBB5_1:
     Err(_) => handle_alloc_error(layout), (liballoc/raw_vec.rs:185)
     mov     edi, 100
     mov     esi, 1
     call    qword, ptr, [rip, +, _ZN5alloc5alloc18handle_alloc_error17h7f6a7a00afe6ad92E@GOTPCREL]
     ud2
