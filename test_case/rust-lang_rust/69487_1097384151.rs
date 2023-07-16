
> error[E0308]: mismatched types
>  --> <anon>:2:16
>   |
> 2 |     [9; || [9; []]];
>   |                ^^ expected `usize`, found array of 0 elements
>   |
>   = note: expected type `usize`
>             found array `[_; 0]`
> 
> error[E0080]: it is undefined behavior to use this value
>  --> <anon>:2:9
>   |
> 2 |     [9; || [9; []]];
>   |         ^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
>   |
>   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
>   = note: the raw bytes of the constant (size: 8, align: 8) {
>               __ __ __ __ __ __ __ __                         │ ░░░░░░░░
>           }
> 