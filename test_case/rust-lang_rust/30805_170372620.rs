 asm
    f460:       f3 0f 6f 52 90          movdqu xmm2,XMMWORD PTR [rdx-0x70]
    f465:       f3 0f 6f 5a a0          movdqu xmm3,XMMWORD PTR [rdx-0x60]
    f46a:       f3 0f 6f 62 b0          movdqu xmm4,XMMWORD PTR [rdx-0x50]
    f46f:       f3 0f 6f 6a c0          movdqu xmm5,XMMWORD PTR [rdx-0x40]
    f474:       66 0f fe d0             paddd  xmm2,xmm0
    f478:       66 0f fe d9             paddd  xmm3,xmm1
    f47c:       66 0f fe d4             paddd  xmm2,xmm4
    f480:       66 0f fe dd             paddd  xmm3,xmm5
    f484:       f3 0f 6f 62 d0          movdqu xmm4,XMMWORD PTR [rdx-0x30]
    f489:       f3 0f 6f 6a e0          movdqu xmm5,XMMWORD PTR [rdx-0x20]
    f48e:       66 0f fe e2             paddd  xmm4,xmm2
    f492:       66 0f fe eb             paddd  xmm5,xmm3
    f496:       f3 0f 6f 42 f0          movdqu xmm0,XMMWORD PTR [rdx-0x10]
    f49b:       f3 0f 6f 0a             movdqu xmm1,XMMWORD PTR [rdx]
    f49f:       66 0f fe c4             paddd  xmm0,xmm4
    f4a3:       66 0f fe cd             paddd  xmm1,xmm5
    f4a7:       48 83 ea 80             sub    rdx,0xffffffffffffff80
    f4ab:       48 83 c3 e0             add    rbx,0xffffffffffffffe0
    f4af:       75 af                   jne    f460 <_ZN11sum_deque_220hbee41beab497848aadaE+0x2b0>
