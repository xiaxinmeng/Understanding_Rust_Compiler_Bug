
$ RUSTFLAGS='-Z trans-time-graph -Z thinlto -C codegen-units=16' cargo +nightly rustc --release -- --emit llvm-ir
