diff
2c2,5
<         test    rdx, rdx
---
>         mov     r9, qword ptr [rdi + 8]
>         cmp     r9, rdx
>         cmova   r9, rdx
>         test    r9, r9
