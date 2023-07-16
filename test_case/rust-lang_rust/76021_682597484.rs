
> cd /tmp
> cargo new app
     Created binary (application) `app` package
> cd app
> echo 1.46.0 > rust-toolchain
> echo 'rand = "*"' >> Cargo.toml
> cargo build
info: syncing channel updates for '1.46.0-x86_64-unknown-linux-musl'
info: latest update on 2020-08-27, rust version 1.46.0 (04488afe3 2020-08-24)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: Defaulting to 500.0 MiB unpack ram
info: installing component 'clippy'
info: installing component 'rust-std'
info: installing component 'rustc'
info: installing component 'rustfmt'
    Updating crates.io index
   Compiling libc v0.2.76
   Compiling getrandom v0.1.14
   Compiling cfg-if v0.1.10
   Compiling ppv-lite86 v0.2.9
   Compiling rand_core v0.5.1
   Compiling rand_chacha v0.2.2
   Compiling rand v0.7.3
   Compiling app v0.1.0 (/tmp/app)
    Finished dev [unoptimized + debuginfo] target(s) in 10.82s
> ldd target/debug/app
	ldd (0x7f321a2c1000)
> file target/debug/app
target/debug/app: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, with debug_info, not stripped
> ./target/debug/app
Hello, world!
