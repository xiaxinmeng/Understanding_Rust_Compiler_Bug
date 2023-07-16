 rust
>    select!(
>       x = rx.recv() => Some(x),
>       _ = later(delay) => None,
>    )
> 