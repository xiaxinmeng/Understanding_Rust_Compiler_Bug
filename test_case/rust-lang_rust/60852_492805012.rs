plain
travis_time:end:176d5e50:start=1557944418431642842,finish=1557944503767835817,duration=85336192975
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:03:12]    Compiling libc v0.2.54
[00:03:12]    Compiling autocfg v0.1.2
[00:03:14]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:16]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:16]    Compiling backtrace v0.3.16 (https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9)
[00:03:19]    Compiling compiler_builtins v0.1.12
[00:03:19]    Compiling backtrace-sys v0.1.28 (https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9)
[00:03:22]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:22]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:23]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:03:23]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
---
[00:04:04]     Checking owning_ref v0.3.3
[00:04:04]    Compiling itertools v0.8.0
[00:04:04]     Checking rustc-hash v1.0.1
[00:04:04]     Checking chalk-macros v0.1.0
[00:04:05]    Compiling backtrace v0.3.16 (https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9)
[00:04:06]     Checking rustc_cratesio_shim v0.0.0 (/checkout/src/librustc_cratesio_shim)
[00:04:06]     Checking ena v0.13.0
[00:04:06]     Checking crossbeam-epoch v0.3.1
[00:04:06]     Checking smallvec v0.6.7
[00:04:06]     Checking smallvec v0.6.7
[00:04:07]     Checking lock_api v0.1.3
[00:04:07]     Checking polonius-engine v0.7.0
[00:04:07]     Checking chalk-engine v0.9.0
[00:04:08]    Compiling rustc_version v0.2.3
[00:04:10]    Compiling miniz-sys v0.1.11
[00:04:11]    Compiling backtrace-sys v0.1.28 (https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9)
[00:04:12]     Checking serialize v0.0.0 (/checkout/src/libserialize)
[00:04:12]     Checking rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
[00:04:13]     Checking num_cpus v1.8.0
[00:04:13]     Checking jobserver v0.1.13
---
[00:07:10] configure: build.locked-deps    := True
[00:07:10] configure: llvm.ccache          := sccache
[00:07:10] configure: build.cargo-native-static := True
[00:07:10] configure: dist.missing-tools   := True
[00:07:10] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:07:10] configure: writing `config.toml` in current directory
[00:07:10] configure: 
[00:07:10] configure: run `python /checkout/x.py --help`
[00:07:10] configure: 
---
[00:08:30]     Checking core v0.0.0 (/checkout/src/libcore)
[00:08:30]    Compiling libc v0.2.54
[00:08:30]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:08:31]    Compiling autocfg v0.1.2
[00:08:33]    Compiling backtrace v0.3.16 (https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9)
[00:08:36]    Compiling compiler_builtins v0.1.12
[00:08:36]    Compiling backtrace-sys v0.1.28 (https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9)
[00:08:55]     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:08:56]     Checking alloc v0.0.0 (/checkout/src/liballoc)
[00:08:56]     Checking rustc-demangle v0.1.10
[00:08:56]     Checking cfg-if v0.1.8
[00:08:56]     Checking cfg-if v0.1.8
[00:08:56]     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:09:00]     Checking rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:09:00]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:09:00]     Checking hashbrown v0.3.0
[00:09:01] error[E0433]: failed to resolve: use of undeclared type or module `OsString`
[00:09:01]    --> src/libstd/sys_common/backtrace.rs:208:28
[00:09:01]     |
[00:09:01] 208 |                 path_buf = OsString::from_wide(wide);
[00:09:01]     |                            ^^^^^^^^ use of undeclared type or module `OsString`
[00:09:02] error: unused import: `crate::os::windows::prelude::*`
[00:09:02]    --> src/libstd/sys_common/backtrace.rs:207:21
[00:09:02]     |
[00:09:02] 207 |                 use crate::os::windows::prelude::*;
[00:09:02] 207 |                 use crate::os::windows::prelude::*;
[00:09:02]     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:09:02]     |
[00:09:02]     = note: `-D unused-imports` implied by `-D warnings`
[00:09:02] 
[00:09:03] error[E0599]: no function or associated item named `from_utf8` found for type `str` in the current scope
[00:09:03]    --> src/libstd/sys_common/backtrace.rs:203:32
[00:09:03]     |
[00:09:03] 203 |                 Path::new(str::from_utf8(bytes).unwrap_or("<unknown>"))
[00:09:03]     |                                ^^^^^^^^^ function or associated item not found in `str`
[00:09:04] error: aborting due to 3 previous errors
[00:09:04] 
[00:09:04] Some errors occurred: E0433, E0599.
[00:09:04] For more information about an error, try `rustc --explain E0433`.
---
travis_time:end:000bda3c:start=1557945058715263970,finish=1557945058720205530,duration=4941560
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0365f10c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04a5a9ed
travis_time:start:04a5a9ed
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:024f56c4
$ dmesg | grep -i kill
