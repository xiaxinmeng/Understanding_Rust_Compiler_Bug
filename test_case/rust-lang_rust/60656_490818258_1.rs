
$ cargo asm foo::serialize
foo::serialize:
 push    r15
 push    r14
 push    rbx
 sub     rsp, 96
 mov     r15, rdx
 mov     rbx, rdi
 xorps   xmm0, xmm0
 movaps  xmmword, ptr, [rsp, +, 80], xmm0
 lea     rdx, [rsp, +, 80]
 mov     qword, ptr, [rsp, +, 56], rdx
 lea     r14, [rsp, +, 72]
 mov     eax, 16
 movq    xmm0, rax
 movdqu  xmmword, ptr, [rsp, +, 64], xmm0
 mov     qword, ptr, [rsp, +, 8], rsi
 lea     rdi, [rsp, +, 32]
 lea     r8, [rsp, +, 8]
 mov     ecx, 16
 mov     r9d, 8
 mov     rsi, r14
 call    qword, ptr, [rip, +, _ZN3std2io6cursor11slice_write17h3193265db7206f0bE@GOTPCREL]
 cmp     qword, ptr, [rsp, +, 32], 1
 je      .LBB5_3
 mov     qword, ptr, [rsp, +, 8], r15
 mov     rdx, qword, ptr, [rsp, +, 56]
 mov     rcx, qword, ptr, [rsp, +, 64]
 lea     rdi, [rsp, +, 32]
 lea     r8, [rsp, +, 8]
 mov     r9d, 8
 mov     rsi, r14
 call    qword, ptr, [rip, +, _ZN3std2io6cursor11slice_write17h3193265db7206f0bE@GOTPCREL]
 cmp     qword, ptr, [rsp, +, 32], 1
 je      .LBB5_3
 movaps  xmm0, xmmword, ptr, [rsp, +, 80]
 movups  xmmword, ptr, [rbx], xmm0
 mov     rax, rbx
 add     rsp, 96
 pop     rbx
 pop     r14
 pop     r15
 ret
.LBB5_3:
 movups  xmm0, xmmword, ptr, [rsp, +, 40]
 movaps  xmmword, ptr, [rsp, +, 16], xmm0
 lea     rdi, [rsp, +, 16]
 call    core::result::unwrap_failed
 ud2
