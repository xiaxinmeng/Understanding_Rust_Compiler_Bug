console
$ touch src/main.rs && time rustup run stable cargo build --release                                                                                                      ~/tmp/hello_nightly 1s (master*)
   Compiling hello_nightly v0.1.0 (file:///Users/gib/tmp/hello_nightly)
    Finished release [optimized] target(s) in 0.25 secs
rustup run stable cargo build --release  0.29s user 0.15s system 93% cpu 0.478 total
$ touch src/main.rs && time rustup run nightly cargo build --release                                                                                                        ~/tmp/hello_nightly (master*)
   Compiling hello_nightly v0.1.0 (file:///Users/gib/tmp/hello_nightly)
    Finished release [optimized] target(s) in 0.25 secs
rustup run nightly cargo build --release  0.29s user 0.16s system 93% cpu 0.482 total
