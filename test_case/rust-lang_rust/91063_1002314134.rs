
> error[E0277]: cannot add-assign `char` to `u32`
>  --> src/main.rs:5:10
>   |
> 5 |   output += c;
>   |          ^^ no implementation for `u32 += char`
>   |
>   = help: the trait `AddAssign<char>` is not implemented for `u32`
> 