rust
>     while {
>         let d = data.read().unwrap();
>         d[0].is_some() || d[1].is_some()
>     } {
> 