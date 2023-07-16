
> error[E0382]: borrow of moved value: `var`
>  --> src/main.rs:7:18
>   |
> 5 |     let closure = |var| {
>   |                    --- move occurs because `var` has type `&mut i32`, which does not implement the `Copy` trait
> 6 |         consumer(var);
>   |                  --- value moved here
> 7 |         consumer(var);
>   |                  ^^^ value borrowed here after move
> 