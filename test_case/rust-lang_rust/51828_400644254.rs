plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0dc5a248
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:02:59]       Memory: 8 GB
[00:02:59]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:59]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:59]       SMC Version (system): 2.8f0
[00:02:59]       Serial Number (system): VM9KbraAWGzr
[00:02:59] 
[00:02:59] hw.ncpu: 4
[00:02:59] hw.byteorder: 1234
[00:02:59] hw.memsize: 8589934592
---
tidy check
[00:05:29] * 554 error codes
[00:05:29] * highest error code: E0709
[00:05:30] * 208 features
[00:05:30] tidy error: /Users/travis/build/rust-lang/rust/src/libcore/ptr.rs:232: platform-specific cfg: cfg(target_os = "macos")
[00:05:31] some tidy checks failed
[00:05:31] 
[00:05:31] 
[00:05:31] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/tidy" "/Users/travis/build/rust-lang/rust/src" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:31] 
[00:05:31] 
[00:05:31] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:31] Build completed unsuccessfully in 0:02:31
[00:05:31] Build completed unsuccessfully in 0:02:31
[00:05:31] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07cabef1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0ee1a47a:start=1530100163745474000,finish=1530100163774097000,duration=28623000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b8169a0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:069df512
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.5

Done. Your build exited with 1.
