log
(gdb) x/20i  0x00007ffff7fee6da 
=> 0x7ffff7fee6da:      movlpd (%rdi),%xmm1
   0x7ffff7fee6de:      movlpd (%rsi),%xmm2
   0x7ffff7fee6e2:      movhpd 0x8(%rdi),%xmm1
   0x7ffff7fee6e7:      movhpd 0x8(%rsi),%xmm2
   0x7ffff7fee6ec:      pxor   %xmm0,%xmm0
   0x7ffff7fee6f0:      pcmpeqb %xmm1,%xmm0
   0x7ffff7fee6f4:      pcmpeqb %xmm2,%xmm1
   0x7ffff7fee6f8:      psubb  %xmm0,%xmm1
   0x7ffff7fee6fc:      pmovmskb %xmm1,%edx
   0x7ffff7fee700:      sub    $0xffff,%edx
   0x7ffff7fee706:      jne    0x7ffff7fefae0
   0x7ffff7fee70c:      add    $0x10,%rsi
   0x7ffff7fee710:      add    $0x10,%rdi
   0x7ffff7fee714:      data16 nopw %cs:0x0(%rax,%rax,1)
   0x7ffff7fee71f:      nop
   0x7ffff7fee720:      and    $0xfffffffffffffff0,%rsi
   0x7ffff7fee724:      and    $0xfffffffffffffff0,%rdi
   0x7ffff7fee728:      mov    $0xffff,%edx
--Type <RET> for more, q to quit, c to continue without paging--q
Quit
(gdb) i r
rax            0x20                32
rbx            0x7ffff7fd0a6c      140737353943660
rcx            0x2c                44
rdx            0x2c925             182565
rsi            0x7ffff7fd0a6c      140737353943660
rdi            0x2e0               736
rbp            0x7fffffffe3e0      0x7fffffffe3e0
rsp            0x7fffffffe148      0x7fffffffe148
r8             0x7ffff7ffe070      140737354129520
r9             0x7ffff7ffe050      140737354129488
r10            0x0                 0
r11            0x39180             233856
r12            0x7ffff7ffe070      140737354129520
r13            0x4f4c5f4543415254  5714046778312053332
r14            0x7ffff7ffe1a0      140737354129824
r15            0x0                 0
rip            0x7ffff7fee6da      0x7ffff7fee6da
eflags         0x10287             [ CF PF SF IF RF ]
cs             0x33                51
ss             0x2b                43
ds             0x0                 0
es             0x0                 0
fs             0x0                 0
gs             0x0                 0
