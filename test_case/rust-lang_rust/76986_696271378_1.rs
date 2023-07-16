assembly
> example::sum_c:
>         mov     rax, qword ptr [rsi]
>         add     rax, qword ptr [rdi]
>         mov     edx, dword ptr [rsi + 8]
>         add     edx, dword ptr [rdi + 8]
>         mov     cl, byte ptr [rsi + 12]
>         add     cl, byte ptr [rdi + 12]
>         movzx   ecx, cl
>         shl     rcx, 32
>         or      rdx, rcx
>         ret
> 