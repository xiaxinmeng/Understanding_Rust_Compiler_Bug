
$ cargo build --target wasm32-unknown-unknown
   Compiling crappy-chess-minimax v0.1.0 (file:///home/jD91mZM2/Coding/Rust/crappy-chess-minimax)
   Compiling crappy-chess-minimax-wasm v0.1.0 (file:///home/jD91mZM2/Coding/Web/jD91mZM2.github.io/crappy-chess-minimax-wasm)
LLVM ERROR: out of memory
error: Could not compile `crappy-chess-minimax-wasm`.

To learn more, run the command again with --verbose.
$ cargo build --target wasm32-unknown-unknown --release
   Compiling crappy-chess-minimax v0.1.0 (file:///home/jD91mZM2/Coding/Rust/crappy-chess-minimax)
   Compiling crappy-chess-minimax-wasm v0.1.0 (file:///home/jD91mZM2/Coding/Web/jD91mZM2.github.io/crappy-chess-minimax-wasm)
    Finished release [optimized] target(s) in 4.66 secs
cargo build --target wasm32-unknown-unknown --release  7.58s user 0.08s system 146% cpu 5.226 total
$ rustc --version
rustc 1.25.0 (84203cac6 2018-03-25)
