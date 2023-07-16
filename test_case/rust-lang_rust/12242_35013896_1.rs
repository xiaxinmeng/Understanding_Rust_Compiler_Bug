 rust
>    bh.iter(|| {
>            range(0, 1000).fold(0, |old, new| old ^ new) // <-- note lack of semicolon
>        });
> 