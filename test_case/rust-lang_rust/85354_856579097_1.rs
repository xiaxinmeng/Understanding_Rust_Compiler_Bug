asm
file /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/iter/range.rs does not exist!
file /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/cmp.rs does not exist!
 pub fn foo_1a(aux_sieve: &mut [u64], aux_primes: &[u32]) {
 push    rax
 mov     r8, rdx
 xor     r9d, r9d
 movabs  r10, 3777893186295716171
.LBB0_1:
 mov     rax, r9
 mul     r10
 shr     rdx, 10
 imul    rax, rdx, 5000
 mov     r11, r9
 sub     r11, rax
 cmp     r11, rcx
 while j < 4096 {
 jae     .LBB0_7
 add     r9, 1
 xor     edx, edx
.LBB0_3:
 aux_sieve[j as usize] |= 1234;
 mov     eax, edx
 aux_sieve[j as usize] |= 1234;
 cmp     rax, rsi
 jae     .LBB0_9
 or      word, ptr, [rdi, +, 8*rax], 1234
 j += aux_primes[i%5000];
 add     edx, dword, ptr, [r8, +, 4*r11]
 while j < 4096 {
 cmp     edx, 4096
 while j < 4096 {
 jb      .LBB0_3
     cmp     r9, 10000000
     jne     .LBB0_1
 }
 pop     rax
 ret
.LBB0_7:
 aux_sieve[j as usize] |= 1234;
 test    rsi, rsi
 je      .LBB0_8
 or      word, ptr, [rdi], 1234
 j += aux_primes[i%5000];
 lea     rdx, [rip, +, .L__unnamed_1]
 mov     rdi, r11
 mov     rsi, rcx
 call    qword, ptr, [rip, +, _ZN4core9panicking18panic_bounds_check17hd3a5ad32cddc7d96E@GOTPCREL]
 ud2
.LBB0_8:
 xor     edx, edx
.LBB0_9:
 aux_sieve[j as usize] |= 1234;
 mov     edi, edx
 aux_sieve[j as usize] |= 1234;
 lea     rdx, [rip, +, .L__unnamed_2]
 call    qword, ptr, [rip, +, _ZN4core9panicking18panic_bounds_check17hd3a5ad32cddc7d96E@GOTPCREL]
 ud2

