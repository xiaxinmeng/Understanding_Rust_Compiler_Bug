 rust
> asm!("
>     .align 16, 0x90
> XOAEUSTNH:
>     dec $0
>     cmpb $$62, ($0)
>     jne XOAEUSTNH" : "+r"(cur));
> 