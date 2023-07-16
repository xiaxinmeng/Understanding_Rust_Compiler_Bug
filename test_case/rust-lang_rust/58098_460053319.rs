plain
travis_time:end:048b05d5:start=1549199867996728845,finish=1549199868970358857,duration=973630012
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:26:03]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:26:03]    Compiling rustc-demangle v0.1.10
[00:26:03]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:26:09]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:26:10] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:26:10]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/arch/x86.rs:82:1
[00:26:10] 82 | #[allow_internal_unstable]
[00:26:10]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:26:10] 
[00:26:29]     Finished release [optimized] target(s) in 1m 09s
---
[00:27:34]    Compiling rls-data v0.18.1
[00:27:34] error[E0658]: use of unstable library feature 'libstd_sys_internals': used by the panic! macro
[00:27:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rls-data-0.18.1/src/lib.rs:168:48
[00:27:34]     |
[00:27:34] 168 | #[cfg_attr(feature = "serialize-rustc", derive(RustcDecodable, RustcEncodable))]
[00:27:34]     |
[00:27:34]     = help: add #![feature(libstd_sys_internals)] to the crate attributes to enable
[00:27:34] 
[00:27:34] error[E0658]: use of unstable library feature 'libstd_sys_internals': used by the panic! macro
[00:27:34] error[E0658]: use of unstable library feature 'libstd_sys_internals': used by the panic! macro
[00:27:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rls-data-0.18.1/src/lib.rs:195:48
[00:27:34]     |
[00:27:34] 195 | #[cfg_attr(feature = "serialize-rustc", derive(RustcDecodable, RustcEncodable))]
[00:27:34]     |
[00:27:34]     = help: add #![feature(libstd_sys_internals)] to the crate attributes to enable
[00:27:34] 
[00:27:34] error[E0658]: use of unstable library feature 'libstd_sys_internals': used by the panic! macro
[00:27:34] error[E0658]: use of unstable library feature 'libstd_sys_internals': used by the panic! macro
[00:27:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rls-data-0.18.1/src/lib.rs:248:48
[00:27:34]     |
[00:27:34] 248 | #[cfg_attr(feature = "serialize-rustc", derive(RustcDecodable, RustcEncodable))]
[00:27:34]     |
[00:27:34]     = help: add #![feature(libstd_sys_internals)] to the crate attributes to enable
[00:27:34] 
[00:27:34] error[E0658]: use of unstable library feature 'libstd_sys_internals': used by the panic! macro
[00:27:34] error[E0658]: use of unstable library feature 'libstd_sys_internals': used by the panic! macro
[00:27:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rls-data-0.18.1/src/lib.rs:284:48
[00:27:34]     |
[00:27:34] 284 | #[cfg_attr(feature = "serialize-rustc", derive(RustcDecodable, RustcEncodable))]
[00:27:34]     |
[00:27:34]     = help: add #![feature(libstd_sys_internals)] to the crate attributes to enable
[00:27:34] 
[00:27:34] error[E0658]: use of unstable library feature 'libstd_sys_internals': used by the panic! macro
[00:27:34] error[E0658]: use of unstable library feature 'libstd_sys_internals': used by the panic! macro
[00:27:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rls-data-0.18.1/src/lib.rs:313:48
[00:27:34]     |
[00:27:34] 313 | #[cfg_attr(feature = "serialize-rustc", derive(RustcDecodable, RustcEncodable))]
[00:27:34]     |
[00:27:34]     = help: add #![feature(libstd_sys_internals)] to the crate attributes to enable
[00:27:34] 
[00:27:34] error: aborting due to 5 previous errors
---
[00:27:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:27:34] expected success, got: exit code: 101
[00:27:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:27:34] Build completed unsuccessfully in 0:23:18
[00:27:34] make: *** [all] Error 1
[00:27:34] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17d9cf67
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 13:45:35 UTC 2019
---
travis_time:end:06b882ce:start=1549201536152487087,finish=1549201536158258675,duration=5771588
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0320cd20
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:288e30eb
travis_time:start:288e30eb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14bb385c
$ dmesg | grep -i kill
