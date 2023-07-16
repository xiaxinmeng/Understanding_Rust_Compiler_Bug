assembly
> example::eq:
>         mov     eax, dword ptr [rdi]
>         cmp     eax, dword ptr [rsi]
>         sete    al
>         ret
> 