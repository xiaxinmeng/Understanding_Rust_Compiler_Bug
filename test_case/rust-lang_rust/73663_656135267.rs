
> RUST_BACKTRACE=1 cargo test --release --features std --verbose
>     Updating crates.io index
> error: failed to get `libtock` as a dependency of package `ctap2 v0.1.0 (/home/joshua/src/OpenSK)`
> 
> Caused by:
>   failed to load source for dependency `libtock`
> 
> Caused by:
>   Unable to update /home/joshua/src/OpenSK/third_party/libtock-rs
> 
> Caused by:
>   failed to read `/home/joshua/src/OpenSK/third_party/libtock-rs/Cargo.toml`
> 
> Caused by:
>   No such file or directory (os error 2)
> 