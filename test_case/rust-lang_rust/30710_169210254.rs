 sh
rustc: x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd
rustc: x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-musl/lib/libstd
warning: dropping unsupported crate type `dylib` for target `arm-unknown-linux-musl`
../src/libstd/sys/unix/fs.rs:188:9: 188:25 error: mismatched types:
 expected `u32`,
    found `u64`
(expected u32,
    found u64) [E0308]
../src/libstd/sys/unix/fs.rs:188         self.entry.d_ino
                                         ^~~~~~~~~~~~~~~~
../src/libstd/sys/unix/fs.rs:188:9: 188:25 help: run `rustc --explain E0308` to see a detailed explanation
