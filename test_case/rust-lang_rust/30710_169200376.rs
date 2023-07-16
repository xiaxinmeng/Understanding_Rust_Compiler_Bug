 sh
[..]
rustc: x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-musl/lib/libstd
rustc: x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd
warning: dropping unsupported crate type `dylib` for target `arm-unknown-linux-musl`
../src/libstd/dynamic_lib.rs:111:37: 111:56 error: mismatched types:
 expected `*const i8`,
    found `*const u8`
(expected i8,
    found u8) [E0308]
[..]
