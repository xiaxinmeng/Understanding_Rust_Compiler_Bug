rust
> fn main() {
>     let a = loop {};
>     let b = while true {};
>     let c = for _ in &[()] {};
>     let d = match loop {} {};
>     let e = return;
>     f(return);
> }
> 