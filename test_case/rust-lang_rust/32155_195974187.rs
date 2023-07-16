 asm
       │ e0:   mov    BYTE PTR [rdx+rcx*1],0x0                                                                                                                                    ▒
 28,57 │       lea    rsi,[rdi+rcx*1]                                                                                                                                             ▒
       │       mov    QWORD PTR [rbx+0x10],rsi                                                                                                                                    ▒
       │       mov    BYTE PTR [rdx+rcx*1+0x1],0x0                                                                                                                                ▒
       │       lea    rsi,[rdi+rcx*1+0x1]                                                                                                                                         ▒
       │       mov    QWORD PTR [rbx+0x10],rsi                                                                                                                                    ▒
       │       mov    BYTE PTR [rdx+rcx*1+0x2],0x0                                                                                                                                ▒
 14,29 │       lea    rsi,[rdi+rcx*1+0x2]                                                                                                                                         ▒
       │       mov    QWORD PTR [rbx+0x10],rsi                                                                                                                                    ▒
       │       mov    BYTE PTR [rdx+rcx*1+0x3],0x0                                                                                                                                ▒
 28,57 │       lea    rsi,[rdi+rcx*1+0x3]                                                                                                                                         ▒
       │       mov    QWORD PTR [rbx+0x10],rsi                                                                                                                                    ▒
 14,29 │       add    rcx,0x4                                                                                                                                                     ▒
       │       cmp    r8,rcx                                                                                                                                                      ▒
       │     ↑ jne    e0      
