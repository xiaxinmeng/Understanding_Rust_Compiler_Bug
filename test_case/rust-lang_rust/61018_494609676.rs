plain
[00:03:56]    Compiling textwrap v0.11.0
[00:03:57] error: failed to run custom build command for `rand_chacha v0.1.1`
[00:03:57] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/debug/build/rand_chacha-264db25efa48e2e1/build-script-build` (exit code: 101)
[00:03:57] --- stdout
[00:03:57] cargo:rerun-if-changed=build.rs
[00:03:57] --- stderr
[00:03:57] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
[00:03:57] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:03:57] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
[00:03:57] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
[00:03:57] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:03:57] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { kind: Other("could not probe for `std`") }', src/libcore/result.rs:997:5
[00:03:57] 
[00:03:57] warning: build failed, waiting for other jobs to finish...
[00:03:57] error: failed to run custom build command for `rand_pcg v0.1.2`
[00:03:57] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/debug/build/rand_pcg-77510ee48e3cc776/build-script-build` (exit code: 101)
[00:03:57] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/debug/build/rand_pcg-77510ee48e3cc776/build-script-build` (exit code: 101)
[00:03:57] --- stdout
[00:03:57] cargo:rerun-if-changed=build.rs
[00:03:57] --- stderr
[00:03:57] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
[00:03:57] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:03:57] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
[00:03:57] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
[00:03:57] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:03:57] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { kind: Other("could not probe for `std`") }', src/libcore/result.rs:997:5
[00:03:57] 
[00:03:57] warning: build failed, waiting for other jobs to finish...
[00:03:57] warning: build failed, waiting for other jobs to finish...
[00:03:59] error: failed to compile `cargo-vendor v0.1.22`, intermediate artifacts can be found at `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools`
[00:03:59] Caused by:
[00:03:59]   build failed
[00:03:59] 
[00:03:59] 
[00:03:59] 
[00:03:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "install" "-j" "4" "--locked" "--color" "always" "--force" "--debug" "--vers" "0.1.22" "cargo-vendor"
[00:03:59] 
[00:03:59] 
[00:03:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:03:59] Build completed unsuccessfully in 0:00:34
---
travis_time:end:1a08069c:start=1558485801426579190,finish=1558485801433045677,duration=6466487
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03679e12
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07ce4d3c
travis_time:start:07ce4d3c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08089b54
$ dmesg | grep -i kill
