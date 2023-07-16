rust
> let x: i32 = "I am not a number!";
> //     ~~~   ~~~~~~~~~~~~~~~~~~~~
> //      |             |
> //      |    initializing expression;
> //      |    compiler infers type `&str`
> //      |
> //    type `i32` assigned to variable `x`
> 