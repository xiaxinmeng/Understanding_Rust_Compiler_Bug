asm
file /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/cmp.rs does not exist!
file /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/num/uint_macros.rs does not exist!
file /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/iter/range.rs does not exist!
 pub fn foo_1b(aux: &mut Aux) {
 xor     ecx, ecx
 movabs  r8, 3777893186295716171
.LBB1_2:
 mov     rax, rcx
 mul     r8
 shr     rdx, 10
 imul    rdx, rdx, 5000
 mov     rax, rcx
 sub     rax, rdx
 xor     edx, edx
.LBB1_3:
 aux.sieve[j as usize] |= 1234;
 mov     esi, edx
 aux.sieve[j as usize] |= 1234;
 or      word, ptr, [rdi, +, 8*rsi], 1234
 j += aux.primes[i%5000];
 add     edx, dword, ptr, [rdi, +, 4*rax, +, 32768]
 while j < 4096 {
 cmp     edx, 4096
 while j < 4096 {
 jb      .LBB1_3
     add     rcx, 1
     cmp     rcx, 10000000
     jne     .LBB1_2
 }
 ret
