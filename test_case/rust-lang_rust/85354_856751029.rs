asm
file /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/cmp.rs does not exist!
file /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/clone.rs does not exist!
file /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/iter/range.rs does not exist!
file /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/fmt/mod.rs does not exist!
file /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/num/uint_macros.rs does not exist!
 pub fn main() {
 push    r15
 push    r14
 push    r13
 push    r12
 push    rbx
 mov     eax, 125664
 call    __rust_probestack
 sub     rsp, rax
 mov     eax, 36
 movaps  xmm0, xmmword, ptr, [rip, +, .LCPI5_0]
.LBB5_1:
 movups  xmmword, ptr, [rsp, +, 4*rax, -, 16], xmm0
 movups  xmmword, ptr, [rsp, +, 4*rax], xmm0
 movups  xmmword, ptr, [rsp, +, 4*rax, +, 16], xmm0
 movups  xmmword, ptr, [rsp, +, 4*rax, +, 32], xmm0
 movups  xmmword, ptr, [rsp, +, 4*rax, +, 48], xmm0
 movups  xmmword, ptr, [rsp, +, 4*rax, +, 64], xmm0
 movups  xmmword, ptr, [rsp, +, 4*rax, +, 80], xmm0
 movups  xmmword, ptr, [rsp, +, 4*rax, +, 96], xmm0
 movups  xmmword, ptr, [rsp, +, 4*rax, +, 112], xmm0
 movups  xmmword, ptr, [rsp, +, 4*rax, +, 128], xmm0
 add     rax, 40
 cmp     rax, 5036
 jne     .LBB5_1
 lea     rdi, [rsp, +, 20128]
 xor     ebx, ebx
 mov     r15, qword, ptr, [rip, +, memset@GOTPCREL]
 let mut aux1 = Aux { sieve, primes };
 mov     edx, 32768
 xor     esi, esi
 call    r15
 let mut aux1 = Aux { sieve, primes };
 lea     rdi, [rsp, +, 52896]
 lea     r14, [rsp, +, 128]
 mov     r12, qword, ptr, [rip, +, memcpy@GOTPCREL]
 let mut aux1 = Aux { sieve, primes };
 mov     edx, 20000
 mov     rsi, r14
 call    r12
 lea     rdi, [rsp, +, 72896]
     mov     edx, 32768
     xor     esi, esi
     call    r15
 #[derive(Clone)]
 lea     rdi, [rsp, +, 105664]
     mov     edx, 20000
     mov     rsi, r14
     call    r12
 let start = Instant::now();
 call    qword, ptr, [rip, +, _ZN3std4time7Instant3now17h4b133928c0a48a41E@GOTPCREL]
 mov     r15, rax
 mov     r14, rdx
 movabs  r12, 3777893186295716171
.LBB5_4:
 mov     rax, rbx
 mul     r12
 shr     rdx, 10
 imul    rax, rdx, 5000
 mov     rcx, rbx
 sub     rcx, rax
 mov     eax, dword, ptr, [rsp, +, 4*rcx, +, 52896]
 xor     ecx, ecx
.LBB5_5:
 data1[j as usize] |= K;
 mov     edx, ecx
 data1[j as usize] |= K;
 or      word, ptr, [rsp, +, 8*rdx, +, 20128], 1234
 j += data2[i % N2];
 add     ecx, eax
 while j < N1B {
 cmp     ecx, 4096
 while j < N1B {
 jb      .LBB5_5
     add     rbx, 1
     cmp     rbx, 10000000
     jne     .LBB5_4
 let end = Instant::now();
 mov     rbx, qword, ptr, [rip, +, _ZN3std4time7Instant3now17h4b133928c0a48a41E@GOTPCREL]
 call    rbx
 mov     qword, ptr, [rsp, +, 64], rax
 mov     qword, ptr, [rsp, +, 72], rdx
 lea     rdi, [rsp, +, 64]
 let duration = end.duration_since(start);
 mov     rsi, r15
 mov     rdx, r14
 call    qword, ptr, [rip, +, _ZN3std4time7Instant14duration_since17hc096810677d2cca4E@GOTPCREL]
 mov     qword, ptr, [rsp, +, 80], rax
 mov     dword, ptr, [rsp, +, 88], edx
 lea     rax, [rsp, +, 80]
 println!("1a {:?}\n", duration);
 mov     qword, ptr, [rsp], rax
 mov     r15, qword, ptr, [rip, +, _ZN57_$LT$core..time..Duration$u20$as$u20$core..fmt..Debug$GT$3fmt17h0e458f4f306000a0E@GOTPCREL]
 mov     qword, ptr, [rsp, +, 8], r15
     lea     rax, [rip, +, .L__unnamed_2]
     mov     qword, ptr, [rsp, +, 16], rax
     mov     qword, ptr, [rsp, +, 24], 2
     mov     qword, ptr, [rsp, +, 32], 0
     mov     r13, rsp
     mov     qword, ptr, [rsp, +, 48], r13
     mov     qword, ptr, [rsp, +, 56], 1
     lea     rdi, [rsp, +, 16]
 call    qword, ptr, [rip, +, _ZN3std2io5stdio6_print17ha58a4cba11598dcaE@GOTPCREL]
 let start = Instant::now();
 call    rbx
 mov     rbx, rax
 mov     r14, rdx
 xor     ecx, ecx
.LBB5_8:
 mov     rax, rcx
 mul     r12
 shr     rdx, 10
 imul    rdx, rdx, 5000
 mov     rax, rcx
 sub     rax, rdx
 xor     edx, edx
.LBB5_9:
 aux.sieve[j as usize] |= K;
 mov     esi, edx
 aux.sieve[j as usize] |= K;
 or      word, ptr, [rsp, +, 8*rsi, +, 72896], 1234
 j += aux.primes[i % N2];
 add     edx, dword, ptr, [rsp, +, 4*rax, +, 105664]
 while j < N1B {
 cmp     edx, 4096
 while j < N1B {
 jb      .LBB5_9
     add     rcx, 1
     cmp     rcx, 10000000
     jne     .LBB5_8
 let end = Instant::now();
 call    qword, ptr, [rip, +, _ZN3std4time7Instant3now17h4b133928c0a48a41E@GOTPCREL]
 mov     qword, ptr, [rsp, +, 96], rax
 mov     qword, ptr, [rsp, +, 104], rdx
 lea     rdi, [rsp, +, 96]
 let duration = end.duration_since(start);
 mov     rsi, rbx
 mov     rdx, r14
 call    qword, ptr, [rip, +, _ZN3std4time7Instant14duration_since17hc096810677d2cca4E@GOTPCREL]
 mov     qword, ptr, [rsp, +, 112], rax
 mov     dword, ptr, [rsp, +, 120], edx
 lea     rax, [rsp, +, 112]
 println!("1b {:?}\n", duration);
 mov     qword, ptr, [rsp], rax
 mov     qword, ptr, [rsp, +, 8], r15
     lea     rax, [rip, +, .L__unnamed_3]
     mov     qword, ptr, [rsp, +, 16], rax
     mov     qword, ptr, [rsp, +, 24], 2
     mov     qword, ptr, [rsp, +, 32], 0
     mov     qword, ptr, [rsp, +, 48], r13
     mov     qword, ptr, [rsp, +, 56], 1
     lea     rdi, [rsp, +, 16]
 call    qword, ptr, [rip, +, _ZN3std2io5stdio6_print17ha58a4cba11598dcaE@GOTPCREL]
 }
 add     rsp, 125664
 pop     rbx
 pop     r12
 pop     r13
 pop     r14
 pop     r15
 ret
