console
$ env CARGO_INCREMENTAL=0 RUSTFLAGS='-C codegen-units=1' cargo b --release --bin souplet
...
   Compiling dataflow v0.1.0 (file:///home/jon/dev/distributary/dataflow)
   Compiling mir v0.1.0 (file:///home/jon/dev/distributary/mir)
   Compiling distributary v0.1.0 (file:///home/jon/dev/distributary)
    Finished release [optimized + debuginfo] target(s) in 10m 36s
