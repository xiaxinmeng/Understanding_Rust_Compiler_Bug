console
$ cargo b -v 
   Compiling a v0.1.0 (/home/lzutao/fork/rust/bug/sized/a)
     Running `rustc --crate-name a --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=146f8215c8690d3c -C extra-filename=-146f8215c8690d3c --out-dir /home/lzutao/.cargo/target/x86_64-unknown-linux-gnu/debug/deps --target x86_64-unknown-linux-gnu -C linker=clang -C incremental=/home/lzutao/.cargo/target/x86_64-unknown-linux-gnu/debug/incremental -L dependency=/home/lzutao/.cargo/target/x86_64-unknown-linux-gnu/debug/deps -L dependency=/home/lzutao/.cargo/target/debug/deps -C link-arg=-fuse-ld=lld -C target-cpu=sandybridge`
