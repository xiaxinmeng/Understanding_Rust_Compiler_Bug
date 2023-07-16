gdb
(gdb) disas
Dump of assembler code for function _ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE:
0x0000555555cf9ad0 <+0>:     push   %r14
0x0000555555cf9ad2 <+2>:     push   %rbx
0x0000555555cf9ad3 <+3>:     push   %rax
0x0000555555cf9ad4 <+4>:     mov    %edi,%r8d
0x0000555555cf9ad7 <+7>:     lea    0x7(%rsi),%rdi
0x0000555555cf9adb <+11>:    and    $0xfffffffffffffff8,%rdi
0x0000555555cf9adf <+15>:    sub    %rsi,%rdi
0x0000555555cf9ae2 <+18>:    je     0x555555cf9b1d <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+77>
0x0000555555cf9ae4 <+20>:    cmp    %rdx,%rdi
0x0000555555cf9ae7 <+23>:    cmovae %rdx,%rdi
0x0000555555cf9aeb <+27>:    test   %rdi,%rdi
0x0000555555cf9aee <+30>:    je     0x555555cf9b1d <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+77>
0x0000555555cf9af0 <+32>:    xor    %ecx,%ecx
0x0000555555cf9af2 <+34>:    data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
0x0000555555cf9b00 <+48>:    cmp    %r8b,(%rsi,%rcx,1)
0x0000555555cf9b04 <+52>:    je     0x555555cf9bad <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+221>
0x0000555555cf9b0a <+58>:    inc    %rcx
0x0000555555cf9b0d <+61>:    cmp    %rcx,%rdi
0x0000555555cf9b10 <+64>:    jne    0x555555cf9b00 <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+48>
0x0000555555cf9b12 <+66>:    lea    -0x10(%rdx),%r9
0x0000555555cf9b16 <+70>:    cmp    %r9,%rdi
0x0000555555cf9b19 <+73>:    jbe    0x555555cf9b23 <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+83>
0x0000555555cf9b1b <+75>:    jmp    0x555555cf9b87 <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+183>
0x0000555555cf9b1d <+77>:    lea    -0x10(%rdx),%r9
0x0000555555cf9b21 <+81>:    xor    %edi,%edi
0x0000555555cf9b23 <+83>:    movabs $0xfefefefefefefeff,%r10
0x0000555555cf9b2d <+93>:    movabs $0x8080808080808080,%r14
0x0000555555cf9b37 <+103>:   movzbl %r8b,%r11d
0x0000555555cf9b3b <+107>:   movabs $0x101010101010101,%rcx
0x0000555555cf9b45 <+117>:   imul   %r11,%rcx
0x0000555555cf9b49 <+121>:   nopl   0x0(%rax)
0x0000555555cf9b50 <+128>:   mov    (%rsi,%rdi,1),%rax
0x0000555555cf9b54 <+132>:   xor    %rcx,%rax
=> 0x0000555555cf9b57 <+135>:   andn   %r14,%rax,%rbx
0x0000555555cf9b5c <+140>:   add    %r10,%rax
0x0000555555cf9b5f <+143>:   test   %rax,%rbx
0x0000555555cf9b62 <+146>:   jne    0x555555cf9b82 <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+178>
0x0000555555cf9b64 <+148>:   mov    0x8(%rsi,%rdi,1),%rax
0x0000555555cf9b69 <+153>:   xor    %rcx,%rax
0x0000555555cf9b6c <+156>:   andn   %r14,%rax,%rbx
0x0000555555cf9b71 <+161>:   add    %r10,%rax
0x0000555555cf9b74 <+164>:   test   %rax,%rbx
0x0000555555cf9b77 <+167>:   jne    0x555555cf9b82 <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+178>
0x0000555555cf9b79 <+169>:   add    $0x10,%rdi
0x0000555555cf9b7d <+173>:   cmp    %r9,%rdi
0x0000555555cf9b80 <+176>:   jbe    0x555555cf9b50 <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+128>
0x0000555555cf9b82 <+178>:   cmp    %rdx,%rdi
0x0000555555cf9b85 <+181>:   ja     0x555555cf9bbd <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+237>
0x0000555555cf9b87 <+183>:   xor    %eax,%eax
0x0000555555cf9b89 <+185>:   cmp    %rdx,%rdi
0x0000555555cf9b8c <+188>:   je     0x555555cf9b9e <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+206>
0x0000555555cf9b8e <+190>:   xchg   %ax,%ax
0x0000555555cf9b90 <+192>:   cmp    %r8b,(%rsi,%rdi,1)
0x0000555555cf9b94 <+196>:   je     0x555555cf9ba3 <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+211>
0x0000555555cf9b96 <+198>:   inc    %rdi
0x0000555555cf9b99 <+201>:   cmp    %rdi,%rdx
0x0000555555cf9b9c <+204>:   jne    0x555555cf9b90 <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+192>
0x0000555555cf9b9e <+206>:   mov    %rdx,%rcx
0x0000555555cf9ba1 <+209>:   jmp    0x555555cf9bb2 <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+226>
0x0000555555cf9ba3 <+211>:   mov    $0x1,%eax
0x0000555555cf9ba8 <+216>:   mov    %rdi,%rcx
0x0000555555cf9bab <+219>:   jmp    0x555555cf9bb2 <_ZN4core5slice6memchr14memchr_aligned17h0e8ed17ea63a570aE+226>
0x0000555555cf9bad <+221>:   mov    $0x1,%eax
0x0000555555cf9bb2 <+226>:   mov    %rcx,%rdx
0x0000555555cf9bb5 <+229>:   add    $0x8,%rsp
0x0000555555cf9bb9 <+233>:   pop    %rbx
0x0000555555cf9bba <+234>:   pop    %r14
0x0000555555cf9bbc <+236>:   ret
0x0000555555cf9bbd <+237>:   lea    0x209a7c(%rip),%rax        # 0x555555f03640
0x0000555555cf9bc4 <+244>:   mov    %rdx,%rsi
0x0000555555cf9bc7 <+247>:   mov    %rax,%rdx
0x0000555555cf9bca <+250>:   call   *0x21c3b8(%rip)        # 0x555555f15f88
0x0000555555cf9bd0 <+256>:   ud2
End of assembler dump.
