 console
$ cargo rustc --release -- -C link-arg=-Wl,-uFOO
$ nm target/release/hello | grep FOO
                 U FOO
