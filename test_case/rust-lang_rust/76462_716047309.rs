compile_fail,E0308
> fn plus_one(x: i32) -> i32 {
>     x + 1
> }
> 
> plus_one("Not a number");
> //       ^^^^^^^^^^^^^^ expected `i32`, found `&str`
> 
> if "Not a bool" {
> // ^^^^^^^^^^^^ expected `bool`, found `&str`
> }
> 