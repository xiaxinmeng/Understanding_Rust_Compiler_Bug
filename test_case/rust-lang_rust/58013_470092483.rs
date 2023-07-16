plain
[01:37:18] [RUSTC-TIMING] clippy_lints test:false 163.881
[01:37:45] [RUSTC-TIMING] cargo_clippy test:false 0.975
[01:37:50] [RUSTC-TIMING] clippy_driver test:false 6.274
[01:37:50]     Finished release [optimized] target(s) in 4m 11s
[01:37:50] duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
[01:37:50] the following dependencies are duplicated although they have the same features enabled:
[01:37:50] the following dependencies are duplicated although they have the same features enabled:
[01:37:50]   serde 1.0.82 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:50]     `clippy-driver` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/i686-unknown-freebsd/release/deps/libserde-1069e9c6d48de2c5.rlib")
[01:37:50]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/i686-unknown-freebsd/release/deps/libserde-a64c22926a7e4d10.rlib")
[01:37:50]   serde_json 1.0.33 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:50]     `clippy-driver` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/i686-unknown-freebsd/release/deps/libserde_json-fde4a5c7a624b957.rlib")
[01:37:50]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/i686-unknown-freebsd/release/deps/libserde_json-d726354b0e14bb6e.rlib")
[01:37:50]   semver 0.9.0 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:50]     `clippy-driver` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/i686-unknown-freebsd/release/deps/libsemver-92852d347d37874a.rlib")
[01:37:50]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/i686-unknown-freebsd/release/deps/libsemver-6e5cc857fa60c66e.rlib")
[01:37:50]   toml 0.4.10 (registry+https://github.com/rust-lang/crates.io-index)
[01:37:50] thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:170:13
[01:37:50]     `clippy-driver` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/i686-unknown-freebsd/release/deps/libtoml-166353e62af0b09b.rlib")
[01:37:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:37:50]     `cargo` ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/i686-unknown-freebsd/release/deps/libtoml-286c63f415e63f28.rlib")
[01:37:50] the following dependencies have different features:
[01:37:50] 
[01:37:50] to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set

[01:37:50] travis_time:end:stage2-clippy-driver:start=1551875720980015109,finish=1551875972647552975,duration=251667537866

[01:37:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host i686-unknown-freebsd --target i686-unknown-freebsd
---
travis_time:end:36fc5954:start=1551875974697244931,finish=1551875974709897720,duration=12652789
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d2262ea
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b2c130e
travis_time:start:2b2c130e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07b42123
$ dmesg | grep -i kill
