
$ rustc --target x86_64-unknown-linux-gnu -C opt-level=3 ./x.rs && ./x
Duration { secs: 2, nanos: 8780924 }

$ rustc --target x86_64-unknown-linux-musl -C opt-level=3 ./x.rs && ./x
Duration { secs: 2, nanos: 364127222 }
