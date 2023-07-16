plain
travis_time:end:15be574b:start=1554171762172935692,finish=1554171837710779949,duration=75537844257
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:16:21]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[01:16:21]    Compiling rustc-demangle v0.1.10
[01:16:22]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[01:16:27]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:16:47] LLVM ERROR: Cannot emit physreg copy instruction
[01:16:47] error: Could not compile `std`.
[01:16:47] To learn more, run the command again with --verbose.
[01:16:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnux32" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:16:47] expected success, got: exit code: 101
[01:16:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target x86_64-fuchsia,aarch64-fuchsia,sparcv9-sun-solaris,wasm32-unknown-unknown,wasm32-unknown-wasi,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-unknown-cloudabi,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda
---
travis_time:end:01c1025f:start=1554176457629802505,finish=1554176457635754400,duration=5951895
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0022b992
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19cac544
travis_time:start:19cac544
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08482c55
$ dmesg | grep -i kill
