
$ cargo +nightly-2018-05-30 test --manifest-path issue50867/Cargo.toml --release -v
   Compiling issue50867 v0.1.0 (file:///Users/servo/simon/simon/issue50867)
     Running `rustc --crate-name issue50867 src/main.rs --emit=dep-info,link -C opt-level=3 --test -C metadata=4a4ef17a80c6bc85 -C extra-filename=-4a4ef17a80c6bc85 --out-dir /Users/servo/simon/simon/issue50867/target/release/deps -L dependency=/Users/servo/simon/simon/issue50867/target/release/deps`
    Finished release [optimized] target(s) in 0.36s
     Running `/Users/servo/simon/simon/issue50867/target/release/deps/issue50867-4a4ef17a80c6bc85`

running 1 test
test test_transform_compress_none ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

