
$ env RUSTFLAGS='-C target-feature=+crt-static' cargo build
...
cannot produce proc-macro for `serde_derive v1.0.87` as the target `x86_64-alpine-linux-musl` does not support these crate types
