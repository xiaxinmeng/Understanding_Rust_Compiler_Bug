
user@Ondo:~/Code/libm_test$ cat Cargo.toml
[package]
name = "libm_test"
version = "0.1.0"
edition = "2021"

[profile.dev]
#lto = "fat"
user@Ondo:~/Code/libm_test$ RUSTC_BOOTSTRAP=1 cargo build --target x86_64-fortanix-unknown-sgx -Zbuild-std
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
user@Ondo:~/Code/libm_test$
