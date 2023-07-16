
$ rustc -Vv
rustc 1.43.0
binary: rustc
commit-hash: unknown
commit-date: unknown
host: powerpc64le-unknown-linux-gnu
release: 1.43.0
LLVM version: 10.0
$ cargo run 
   Compiling range_len v0.1.0 (/range_len)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/range_len`
[src/main.rs:3] &range = -2..9223372036854775807
[src/main.rs:6] range.size_hint() = (
    9223372036854775809,
    Some(
        9223372036854775809,
    ),
)
$ cargo run --release
   Compiling range_len v0.1.0 (/range_len)
    Finished release [optimized] target(s) in 0.30s
     Running `target/release/range_len`
[src/main.rs:3] &range = -2..9223372036854775807
[src/main.rs:6] range.size_hint() = (
    0,
    Some(
        0,
    ),
)
