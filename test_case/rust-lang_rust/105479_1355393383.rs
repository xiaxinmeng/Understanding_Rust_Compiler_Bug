
> error[E0004]: non-exhaustive patterns: `(&_, _)` not covered
>  --> src/main.rs:4:11
>   |
> 4 |     match (a, b) {
>   |           ^^^^^^ pattern `(&_, _)` not covered
>   |
>   = note: the matched value is of type `(&str, &str)`
> help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
>   |
> 6 ~         ("c", "d") => {}
> 7 +         (&_, _) => todo!()
>   |
> 