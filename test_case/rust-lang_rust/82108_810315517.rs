
$ build/x86_64-unknown-linux-gnu/stage1/bin/rustc apple/banana/chaenomeles.rs --remap-path-prefix=apple/banana/=/first/ --remap-path-prefix=apple/=/second/
$ ./chaenomeles thread 'main' panicked at 'explicit panic', /first/chaenomeles.rs:1:12
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
