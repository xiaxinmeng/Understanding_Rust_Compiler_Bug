rust
> pub fn example_f(mut x: String) {
>     x = "foo".to_string(); // this still writes to memory in the caller's stack frame
> }
> 