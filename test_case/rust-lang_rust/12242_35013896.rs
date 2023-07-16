 rust
>     bh.iter(|| {
>             return range(0, 1000).fold(0, |old, new| old ^ new);
>         });
> 