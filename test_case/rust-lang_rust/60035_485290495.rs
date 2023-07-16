plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:05ef0acd
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:07:49]    Compiling scopeguard v0.3.3
[00:07:49]    Compiling rand_core v0.3.0
[00:07:49]    Compiling void v1.0.2
[00:07:50]    Compiling unicode-xid v0.1.0
[00:07:50]    Compiling rustc-rayon-core v0.1.2 (https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb)
[00:07:50]    Compiling stable_deref_trait v1.1.0
[00:07:50]    Compiling rustc-rayon v0.1.2 (https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb)
[00:07:50]    Compiling unicode-width v0.1.5
[00:07:50]    Compiling bitflags v1.0.4
[00:07:50]    Compiling byteorder v1.2.7
[00:07:50]    Compiling graphviz v0.0.0 (/checkout/src/libgraphviz)
---
[00:49:15]    Compiling proc-macro2 v0.4.24
[00:49:15]    Compiling void v1.0.2
[00:49:15]    Compiling unicode-xid v0.1.0
[00:49:15]    Compiling stable_deref_trait v1.1.0
[00:49:16]    Compiling rustc-rayon-core v0.1.2 (https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb)
[00:49:16]    Compiling rustc-rayon v0.1.2 (https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb)
[00:49:16]    Compiling bitflags v1.0.4
[00:49:16]    Compiling byteorder v1.2.7
[00:49:16]    Compiling unicode-width v0.1.5
[00:49:16]    Compiling graphviz v0.0.0 (/checkout/src/libgraphviz)
---
[01:18:39]      |
[01:18:39]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:18:39] 
[01:18:39]     Checking measureme v0.2.1
[01:18:39]     Checking rustc-rayon-core v0.1.2 (https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb)
[01:18:40]     Checking flate2 v1.0.6
[01:18:40]     Checking syn v0.15.22
[01:18:41]     Checking rand_pcg v0.1.1
[01:18:41]     Checking rand_chacha v0.1.0
[01:18:41]     Checking rand_chacha v0.1.0
[01:18:42]     Checking rustc-rayon v0.1.2 (https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb)
[01:18:49]     Checking parking_lot_core v0.4.0
[01:18:50]     Checking tempfile v3.0.5
[01:18:50]     Checking parking_lot v0.7.1
[01:18:51]     Checking synstructure v0.10.1
---
[01:36:45]   Downloaded openssl-src v111.1.0+1.1.1a
[01:36:58] error: failed to sync
[01:36:58] 
[01:36:58] Caused by:
[01:36:58]   found duplicate version of package `rustc-rayon v0.1.2` vendored from two sources:
[01:36:58] 
[01:36:58]  source 1: https://github.com/Zoxc/rayon.git?branch=alt_scope#b6f36bdb
[01:36:58]  source 2: registry `https://github.com/rust-lang/crates.io-index`
[01:36:58] 
[01:36:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "vendor"
[01:36:58] expected success, got: exit code: 101
[01:36:58] 
---
travis_time:end:012cd1ec:start=1555889291497023509,finish=1555889291509186082,duration=12162573
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ca2cc84
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:39c0d230
travis_time:start:39c0d230
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:136f7cca
$ dmesg | grep -i kill
