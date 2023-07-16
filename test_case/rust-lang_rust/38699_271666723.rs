
$ RUSTFLAGS='--verbose' cargo +nightly build --verbose --target i686-unknown-linux-gnu
   Compiling foo v0.1.0 (file:///home/alex/code/foo)
     Running `rustc --crate-name build_script_build build.rs --crate-type bin --emit=dep-info,link -g -C metadata=7603bf8134b87689 -C extra-filename=-7603bf8134b87689 --out-dir /home/alex/code/lol2/target/debug/build/lol2-7603bf8134b87689 -L dependency=/home/alex/code/lol2/target/debug/deps`
...
