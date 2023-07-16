asm
00007ffd`9ac9dfdb 8b0517321700         mov     eax, dword ptr [structopt_derive_afd65372c61b620b!_tls_index (00007ffd`9ae111f8)]
00007ffd`9ac9dfe1 65488b0c2558000000   mov     rcx, qword ptr gs:[58h]
00007ffd`9ac9dfea 488b0cc1             mov     rcx, qword ptr [rcx+rax*8]
00007ffd`9ac9dfee c5fc288120000000     vmovaps ymm0, ymmword ptr [rcx+20h] ds:00000210`4d54ced0=00
00007ffd`9ac9dff6 4883b92000000000     cmp     qword ptr [rcx+20h], 0
00007ffd`9ac9dffe c5fc280d5a051100     vmovaps ymm1, ymmword ptr [structopt_derive_afd65372c61b620b!_ymm (00007ffd`9adae560)]
00007ffd`9ac9e006 488b8130000000       mov     rax, qword ptr [rcx+30h]
