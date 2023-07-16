rust
> fn main() {
>     didnt_check_time_before() // but now does
>     env::set_var("RUST_CLOCK_ASSUME_MONOTONIC", "0");
>     Instant::now() // this is supposed to set up the cache.
> }
> 