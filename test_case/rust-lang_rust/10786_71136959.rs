
$ RUST_LOG=rustc::metadata::loader=info rustc hello.rs 2>&1 | grep reading
INFO:rustc::metadata::loader: rlib reading metadata from: /home/steve/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.rlib
INFO:rustc::metadata::loader: reading libstd-4e7c5e5c.rlib => 0ms
