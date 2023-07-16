asm
check_stage1::foo:
 push    rbx
 mov     rbx, rdi
 mov     edi, 1
 mov     esi, 1
 call    qword, ptr, [rip, +, __rust_alloc@GOTPCREL]
 test    rax, rax
 je      .LBB1_1
 mov     byte, ptr, [rax], 97
 mov     qword, ptr, [rbx], rax
 movaps  xmm0, xmmword, ptr, [rip, +, .LCPI1_0]
 movups  xmmword, ptr, [rbx, +, 8], xmm0
 mov     rax, rbx
 pop     rbx
 ret
.LBB1_1:
 mov     edi, 1
 mov     esi, 1
 call    alloc::raw_vec::RawVec<T,A>::allocate_in::{{closure}}
 ud2
