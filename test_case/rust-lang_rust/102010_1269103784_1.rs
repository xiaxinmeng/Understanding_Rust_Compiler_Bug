
(bash@pop-os) ~/rust-lang/rust2 [17:54:06]
; cargo +r2-stage1 miri test
thread 'main' panicked at 'failed to determine underlying rustc version of Miri: CommandError { stdout: "", stderr: "/home/jnelson/rust-lang/rust2/build/x86_64-unknown-linux-gnu/stage1/bin/miri: error while loading shared libraries: librustc_driver-780f2316356d3243.so: cannot open shared object file: No such file or directory\n" }', src/tools/miri/cargo-miri/src/util.rs:116:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
(bash@pop-os) ~/rust-lang/rust2 [17:54:24]
; fd librustc_driver-780f2316356d3243.so build/x86_64-unknown-linux-gnu/
build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_driver-780f2316356d3243.so
build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-780f2316356d3243.so
build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-780f2316356d3243.so
; LD_LIBRARY_PATH=build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib cargo +r2-stage1 miri setup
Preparing a sysroot for Miri (target: x86_64-unknown-linux-gnu)...
error: could not determine rustc version  # strace shows that it's looking for the same `librustc_driver-780f2316356d3243.so`
note: run with `RUST_BACKTRACE=1` for a backtrace
fatal error: failed to run xargo, see error details above
