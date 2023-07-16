
$ touch src/lib.rs
19:43 vorner@hydra ~/prog/tokio-jsonrpc (master 3 @1) 78%
$ CARGO_INCREMENTAL=0 cargo build --release
   Compiling tokio-jsonrpc v0.9.0 (file:///home/vorner/prog/tokio-jsonrpc)
    Finished release [optimized] target(s) in 1447.22 secs
20:07 vorner@hydra ~/prog/tokio-jsonrpc (master 3 @1) 78%

$ CARGO_INCREMENTAL=0 cargo +stable build --release
   Compiling slog v2.0.6
   Compiling byteorder v1.1.0
   Compiling scoped-tls v0.1.0
   Compiling libc v0.2.29
   Compiling unicode-xid v0.0.4
   Compiling quote v0.3.15
   Compiling num-traits v0.1.40
   Compiling futures v0.1.14
   Compiling itoa v0.3.1
   Compiling slab v0.3.0
   Compiling log v0.3.8
   Compiling cfg-if v0.1.2
   Compiling lazycell v0.5.1
   Compiling serde v1.0.11
   Compiling dtoa v0.4.1
   Compiling synom v0.11.3
   Compiling rand v0.3.16
   Compiling net2 v0.2.30
   Compiling iovec v0.1.0
   Compiling bytes v0.4.4
   Compiling syn v0.11.11
   Compiling mio v0.6.10
   Compiling tokio-io v0.1.2
   Compiling uuid v0.5.1
   Compiling tokio-core v0.1.9
   Compiling serde_json v1.0.2
   Compiling serde_derive_internals v0.15.1
   Compiling serde_derive v1.0.11
   Compiling tokio-jsonrpc v0.9.0 (file:///home/vorner/prog/tokio-jsonrpc)
    Finished release [optimized] target(s) in 95.73 secs
20:10 vorner@hydra ~/prog/tokio-jsonrpc (master 3 @1) 78%

$ rustup run stable rustc --version --verbose
rustc 1.19.0 (0ade33941 2017-07-17)
binary: rustc
commit-hash: 0ade339411587887bf01bcfa2e9ae4414c8900d4
commit-date: 2017-07-17
host: x86_64-unknown-linux-gnu
release: 1.19.0
LLVM version: 4.0
20:12 vorner@hydra ~/prog/tokio-jsonrpc (master 3 @1) 78%

$ rustc --version --verbose
rustc 1.21.0-nightly (215e0b10e 2017-08-08)
binary: rustc
commit-hash: 215e0b10eac17e43f0132971f4e2dd018fc33d43
commit-date: 2017-08-08
host: x86_64-unknown-linux-gnu
release: 1.21.0-nightly
LLVM version: 4.0
20:12 vorner@hydra ~/prog/tokio-jsonrpc (master 3 @1) 78%
