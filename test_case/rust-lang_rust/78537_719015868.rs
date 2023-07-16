
> error: expected iterator, found block
>  --> src/lib.rs
>   |
> 2 |    for i in {
>   |             ^ this starts a block expression ...
> 5 |     1
>   |     ^ ... so there is no body for the loop
>   =     help: try adding an iterator to the for-loop: `for i in iter {`
> 