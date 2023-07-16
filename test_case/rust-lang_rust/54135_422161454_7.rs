 console
$ cargo rustc --release -- -C link-arg=-Wl,-uFOO
$ nm target/release/hello | grep FOO
000000000004e000 R FOO
