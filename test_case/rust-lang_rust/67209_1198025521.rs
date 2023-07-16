
> ~/mwe/a$ cargo build --target x86_64-unknown-linux-musl
> ~/mwe/a$ readelf -S target/x86_64-unknown-linux-musl/debug/a | grep -A1 .mysection
>   [703] .mysection        PROGBITS         00000000000478d0  000488d0
>        0000000000000004  0000000000000000   A       0     0     8
> 