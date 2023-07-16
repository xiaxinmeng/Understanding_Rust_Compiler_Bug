
[00:49:40] --- stderr

[00:49:40] configure: error: 

[00:49:40] *** expat is required. or try to use --enable-libxml2

[00:49:40] make: *** [/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-41d9b2a793d1c36f/out/Makefile] Error 1

[00:49:40] thread 'main' panicked at 'assertion failed: Command::new("make").args(&["-R", "-f",

[00:49:40]                             "makefile.cargo"]).status().unwrap().success()', /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.3/build.rs:17:4

[00:49:40] note: Run with `RUST_BACKTRACE=1` for a backtrace.

[00:49:40] 

[00:49:40] warning: build failed, waiting for other jobs to finish...

[00:50:03] error: build failed

[00:50:03] thread 'main' panicked at 'tests failed for https://github.com/servo/webrender', /checkout/src/tools/cargotest/main.rs:100:8
