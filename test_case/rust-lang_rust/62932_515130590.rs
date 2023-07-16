
~/dev/libc master> xargo +nightly build -vv --no-default-features --target thumbv6m-none-eabi
+ "rustc" "--print" "sysroot"
WARNING: the sysroot can't be built for the Stable channel. Switch to nightly.
+ "cargo" "+nightly" "build" "-vv" "--no-default-features" "--target" "thumbv6m-none-eabi"
   Compiling libc v0.2.60 (/home/jonas/dev/libc)
     Running `CARGO_PKG_VERSION_PATCH=60 CARGO_PKG_HOMEPAGE='https://github.com/rust-lang/libc' CARGO_PKG_AUTHORS='The Rust Project Developers' CARGO=/home/jonas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo CARGO_PRIMARY_PACKAGE=1 CARGO_PKG_VERSION_PRE= CARGO_MANIFEST_DIR=/home/jonas/dev/libc LD_LIBRARY_PATH='/home/jonas/dev/libc/target/debug/deps:/home/jonas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/jonas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib' CARGO_PKG_NAME=libc CARGO_PKG_VERSION_MINOR=2 CARGO_PKG_REPOSITORY='https://github.com/rust-lang/libc' CARGO_PKG_DESCRIPTION='Raw FFI bindings to platform libraries like libc.
' CARGO_PKG_VERSION=0.2.60 CARGO_PKG_VERSION_MAJOR=0 rustc --crate-name build_script_build build.rs --color always --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=73b3627e6214c15a -C extra-filename=-73b3627e6214c15a --out-dir /home/jonas/dev/libc/target/debug/build/libc-73b3627e6214c15a -C incremental=/home/jonas/dev/libc/target/debug/incremental -L dependency=/home/jonas/dev/libc/target/debug/deps`
     Running `/home/jonas/dev/libc/target/debug/build/libc-73b3627e6214c15a/build-script-build`
[libc 0.2.60] cargo:rustc-cfg=libc_priv_mod_use
[libc 0.2.60] cargo:rustc-cfg=libc_union
[libc 0.2.60] cargo:rustc-cfg=libc_const_size_of
[libc 0.2.60] cargo:rustc-cfg=libc_align
[libc 0.2.60] cargo:rustc-cfg=libc_core_cvoid
[libc 0.2.60] cargo:rustc-cfg=libc_packedN
     Running `CARGO_PKG_AUTHORS='The Rust Project Developers' CARGO=/home/jonas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo CARGO_MANIFEST_DIR=/home/jonas/dev/libc LD_LIBRARY_PATH='/home/jonas/dev/libc/target/debug/deps:/home/jonas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/jonas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib' CARGO_PKG_REPOSITORY='https://github.com/rust-lang/libc' CARGO_PKG_VERSION_MINOR=2 CARGO_PKG_VERSION_PATCH=60 CARGO_PKG_HOMEPAGE='https://github.com/rust-lang/libc' OUT_DIR=/home/jonas/dev/libc/target/thumbv6m-none-eabi/debug/build/libc-6bae0f8abe459371/out CARGO_PRIMARY_PACKAGE=1 CARGO_PKG_VERSION_PRE= CARGO_PKG_NAME=libc CARGO_PKG_DESCRIPTION='Raw FFI bindings to platform libraries like libc.
' CARGO_PKG_VERSION=0.2.60 CARGO_PKG_VERSION_MAJOR=0 rustc --crate-name libc src/lib.rs --color always --crate-type lib --emit=dep-info,metadata,link -C debuginfo=2 -C metadata=eea895270b6af526 -C extra-filename=-eea895270b6af526 --out-dir /home/jonas/dev/libc/target/thumbv6m-none-eabi/debug/deps --target thumbv6m-none-eabi -C incremental=/home/jonas/dev/libc/target/thumbv6m-none-eabi/debug/incremental -L dependency=/home/jonas/dev/libc/target/thumbv6m-none-eabi/debug/deps -L dependency=/home/jonas/dev/libc/target/debug/deps --cfg libc_priv_mod_use --cfg libc_union --cfg libc_const_size_of --cfg libc_align --cfg libc_core_cvoid --cfg libc_packedN`
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
