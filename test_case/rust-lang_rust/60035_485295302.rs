plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0e60e48e
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:07:43]    Compiling rand_core v0.3.0
[00:07:43]    Compiling proc-macro2 v0.4.24
[00:07:44]    Compiling lazy_static v1.2.0
[00:07:44]    Compiling unicode-xid v0.1.0
[00:07:44]    Compiling rustc-rayon-core v0.1.2 (https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb)
[00:07:44]    Compiling stable_deref_trait v1.1.0
[00:07:44]    Compiling rustc-rayon v0.1.2 (https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb)
[00:07:44]    Compiling byteorder v1.2.7
[00:07:44]    Compiling either v1.5.0
[00:07:44]    Compiling unicode-width v0.1.5
[00:07:44]    Compiling graphviz v0.0.0 (/checkout/src/libgraphviz)
---
[00:47:00]    Compiling lazy_static v1.2.0
[00:47:01]    Compiling void v1.0.2
[00:47:01]    Compiling scopeguard v0.3.3
[00:47:01]    Compiling unicode-xid v0.1.0
[00:47:01]    Compiling rustc-rayon-core v0.1.2 (https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb)
[00:47:01]    Compiling stable_deref_trait v1.1.0
[00:47:01]    Compiling rustc-rayon v0.1.2 (https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb)
[00:47:01]    Compiling either v1.5.0
[00:47:01]    Compiling byteorder v1.2.7
[00:47:01]    Compiling bitflags v1.0.4
[00:47:01]    Compiling graphviz v0.0.0 (/checkout/src/libgraphviz)
---
[01:15:28]     Checking env_logger v0.5.13
[01:15:29]     Checking measureme v0.2.1
[01:15:29]     Checking backtrace v0.3.11
[01:15:30]     Checking flate2 v1.0.6
[01:15:31]     Checking rustc-rayon-core v0.1.2 (https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb)
[01:15:32]     Checking rand_chacha v0.1.0
[01:15:32]     Checking rand_chacha v0.1.0
[01:15:32]     Checking rustc-rayon v0.1.2 (https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb)
[01:15:39]     Checking synstructure v0.10.1
[01:15:39]  Documenting rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[01:15:40]     Checking parking_lot_core v0.4.0
[01:15:41]     Checking tempfile v3.0.5
---
[01:32:01]   Downloaded openssl-src v111.1.0+1.1.1a
[01:32:13] error: failed to sync
[01:32:13] 
[01:32:13] Caused by:
[01:32:13]   found duplicate version of package `rustc-rayon v0.1.2` vendored from two sources:
[01:32:13] 
[01:32:13]  source 1: https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb
[01:32:13]  source 2: registry `https://github.com/rust-lang/crates.io-index`
[01:32:13] 
[01:32:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "vendor"
[01:32:13] expected success, got: exit code: 101
[01:32:13] 
---
travis_time:end:2655ffe0:start=1555893450024293784,finish=1555893450046205009,duration=21911225
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:025f512c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06d85ea4
travis_time:start:06d85ea4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1465391c
$ dmesg | grep -i kill
