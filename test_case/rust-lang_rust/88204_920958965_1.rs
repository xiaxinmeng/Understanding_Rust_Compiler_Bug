asm
rustc_1_55::test:
        vmovq   xmm0, rsi
        vpbroadcastq    ymm0, xmm0
        vpcmpeqq        ymm1, ymm0, ymmword ptr [rdi + 96]
        vpcmpeqq        ymm2, ymm0, ymmword ptr [rdi + 64]
        vpcmpeqq        ymm3, ymm0, ymmword ptr [rdi + 32]
        vpcmpeqq        ymm0, ymm0, ymmword ptr [rdi]
        vpackssdw       ymm1, ymm2, ymm1
        vpackssdw       ymm0, ymm0, ymm3
        vpermq  ymm1, ymm1, 216
        vpermq  ymm0, ymm0, 216
        vpackssdw       ymm0, ymm0, ymm1
        vpmovmskb       eax, ymm0
        test    eax, -1431655766
        setne   al
        vzeroupper
        ret
        
nightly::test:
        mov     al, 1
        cmp     qword ptr [rdi], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 8], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 16], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 24], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 32], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 40], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 48], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 56], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 64], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 72], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 80], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 88], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 96], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 104], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 112], rsi
        je      .LBB0_16
        cmp     qword ptr [rdi + 120], rsi
        sete    al
.LBB0_16:
        ret
