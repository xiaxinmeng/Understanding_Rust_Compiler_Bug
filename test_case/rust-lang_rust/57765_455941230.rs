plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:09ac31e8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:30:13]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:30:14] error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)`
[00:30:14] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/release/build/rustc_llvm-32ba2d0adcc10f37/build-script-build` (exit code: 101)
[00:30:14] --- stdout
[00:30:14] cargo:rerun-if-changed=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config
[00:30:14] cargo:rerun-if-env-changed=LLVM_CONFIG
[00:30:14] --- stderr
[00:30:14] --- stderr
[00:30:14] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config: relocation error: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config: symbol _ZN4llvm6Triple9normalizeB5cxx11ENS_9StringRefE version LLVM_8 not defined in file libLLVM-8svn.so with link time reference
[00:30:14] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config" "--version"
[00:30:14] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:30:14] 
[00:30:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json"
[00:30:14] expected success, got: exit code: 101
---
travis_time:end:150eff50:start=1548038091862288996,finish=1548038091872647896,duration=10358900
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:025d2977
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3029743d
travis_time:start:3029743d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05b4e950
$ dmesg | grep -i kill
