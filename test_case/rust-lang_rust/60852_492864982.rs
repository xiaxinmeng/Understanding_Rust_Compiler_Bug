plain
travis_time:end:06c1f5c9:start=1557957738017360538,finish=1557957875130159089,duration=137112798551
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:03:11]    Compiling libc v0.2.54
[00:03:11]    Compiling autocfg v0.1.2
[00:03:14]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:14]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:14]    Compiling backtrace v0.3.16 (https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9)
[00:03:17]    Compiling cmake v0.1.38
[00:03:17]    Compiling cmake v0.1.38
[00:03:17]    Compiling backtrace-sys v0.1.28 (https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9)
[00:03:20]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:21]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:03:21]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:22]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
---
[00:04:04]    Compiling semver v0.9.0
[00:04:04]     Checking owning_ref v0.3.3
[00:04:04]    Compiling itertools v0.8.0
[00:04:04]     Checking rustc-hash v1.0.1
[00:04:04]    Compiling backtrace v0.3.16 (https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9)
[00:04:04]     Checking humantime v1.2.0
[00:04:05]     Checking smallvec v0.6.7
[00:04:05]     Checking rustc_cratesio_shim v0.0.0 (/checkout/src/librustc_cratesio_shim)
[00:04:05]     Checking ena v0.13.0
[00:04:05]     Checking ena v0.13.0
[00:04:05]    Compiling miniz-sys v0.1.11
[00:04:05]    Compiling backtrace-sys v0.1.28 (https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9)
[00:04:07]     Checking lock_api v0.1.3
[00:04:07]     Checking polonius-engine v0.7.0
[00:04:07]     Checking chalk-engine v0.9.0
[00:04:09]     Checking serialize v0.0.0 (/checkout/src/libserialize)
---
[00:07:13] configure: build.locked-deps    := True
[00:07:13] configure: llvm.ccache          := sccache
[00:07:13] configure: build.cargo-native-static := True
[00:07:13] configure: dist.missing-tools   := True
[00:07:13] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:07:13] configure: writing `config.toml` in current directory
[00:07:13] configure: 
[00:07:13] configure: run `python /checkout/x.py --help`
[00:07:13] configure: 
---
[00:08:36]     Checking core v0.0.0 (/checkout/src/libcore)
[00:08:36]    Compiling libc v0.2.54
[00:08:36]    Compiling autocfg v0.1.2
[00:08:37]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:08:40]    Compiling backtrace v0.3.16 (https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9)
[00:08:42]    Compiling compiler_builtins v0.1.12
[00:08:42]    Compiling backtrace-sys v0.1.28 (https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9)
[00:09:02]     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:09:04]     Checking alloc v0.0.0 (/checkout/src/liballoc)
[00:09:04]     Checking cfg-if v0.1.8
[00:09:04]     Checking rustc-demangle v0.1.10
[00:09:04]     Checking rustc-demangle v0.1.10
[00:09:04]     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:09:08]     Checking rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:09:08]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:09:08]     Checking hashbrown v0.3.0
[00:09:14] error: constant item is never used: `IMAGE_FILE_MACHINE_I386`
[00:09:14]    --> src/libstd/sys/windows/c.rs:278:1
[00:09:14]     |
[00:09:14] 278 | pub const IMAGE_FILE_MACHINE_I386: DWORD = 0x014c;
[00:09:14]     |
[00:09:14]     = note: `-D dead-code` implied by `-D warnings`
[00:09:14] 
[00:09:14] error: struct is never constructed: `ADDRESS64`
---
[00:09:14]     |
[00:09:14] 655 | pub struct STACKFRAME64 {
[00:09:14]     | ^^^^^^^^^^^^^^^^^^^^^^^
[00:09:14] 
[00:09:14] error: struct is never constructed: `KDHELP64`
[00:09:14]    --> src/libstd/sys/windows/c.rs:671:1
[00:09:14]     |
[00:09:14] 671 | pub struct KDHELP64 {
[00:09:14] 
[00:09:14] 
[00:09:14] error: foreign function is never used: `RtlCaptureContext`
[00:09:14]     --> src/libstd/sys/windows/c.rs:1224:5
[00:09:14]      |
[00:09:14] 1224 |     pub fn RtlCaptureContext(ctx: *mut CONTEXT);
[00:09:14] 
[00:09:14] 
[00:09:14] error: foreign function is never used: `LoadLibraryW`
[00:09:14]     --> src/libstd/sys/windows/c.rs:1256:5
[00:09:14]      |
[00:09:14] 1256 |     pub fn LoadLibraryW(name: LPCWSTR) -> HMODULE;
[00:09:14] 
[00:09:14] 
[00:09:14] error: constant item is never used: `PROCESS_QUERY_INFORMATION`
[00:09:14]     --> src/libstd/sys/windows/c.rs:1369:5
[00:09:14]      |
[00:09:14] 1369 |     pub const PROCESS_QUERY_INFORMATION: DWORD = 0x0400;
[00:09:14] 
[00:09:14] 
[00:09:14] error: constant item is never used: `CP_ACP`
[00:09:14]     --> src/libstd/sys/windows/c.rs:1371:5
[00:09:14]      |
[00:09:14] 1371 |     pub const CP_ACP: UINT = 0;
[00:09:14] 
[00:09:14] 
[00:09:14] error: constant item is never used: `WC_NO_BEST_FIT_CHARS`
[00:09:14]     --> src/libstd/sys/windows/c.rs:1373:5
[00:09:14]      |
[00:09:14] 1373 |     pub const WC_NO_BEST_FIT_CHARS: DWORD = 0x00000400;
[00:09:14] 
[00:09:14] 
[00:09:14] error: foreign function is never used: `OpenProcess`
[00:09:14]     --> src/libstd/sys/windows/c.rs:1376:9
[00:09:14]      |
[00:09:14] 1376 | /         pub fn OpenProcess(dwDesiredAccess: DWORD,
[00:09:14] 1377 | |                            bInheritHandle: BOOL,
[00:09:14] 1378 | |                            dwProcessId: DWORD) -> HANDLE;
[00:09:14] 
[00:09:14] 
[00:09:14] error: function is never used: `QueryFullProcessImageNameW`
[00:09:14]      |
[00:09:14]      |
[00:09:14] 38   | / macro_rules! compat_fn {
[00:09:14] 39   | |     ($module:ident: $(
[00:09:14] 40   | |         pub fn $symbol:ident($($argname:ident: $argtype:ty),*)
[00:09:14] 41   | |                                   -> $rettype:ty {
[00:09:14] ...    |
[00:09:14] 46   | |         pub unsafe fn $symbol($($argname: $argtype),*) -> $rettype {
[00:09:14] ...    |
[00:09:14] 70   | |     )*)
[00:09:14] 71   | | }
[00:09:14]      | |_- in this expansion of `compat_fn!`
[00:09:14]      | |_- in this expansion of `compat_fn!`
[00:09:14]      | 
[00:09:14]     ::: src/libstd/sys/windows/c.rs:1381:5
[00:09:14]      |
[00:09:14] 1381 | /     compat_fn! {
[00:09:14] 1383 | |
[00:09:14] 1383 | |
[00:09:14] 1384 | |         pub fn QueryFullProcessImageNameW(_hProcess: HANDLE,
[00:09:14] 1389 | |         }
[00:09:14] 1390 | |     }
[00:09:14]      | |_____- in this macro invocation
[00:09:14] 
---
[00:09:14] 
[00:09:14] error: method is never used: `open`
[00:09:14]   --> src/libstd/sys/windows/dynamic_lib.rs:12:5
[00:09:14]    |
[00:09:14] 12 |     pub fn open(filename: &str) -> io::Result<DynamicLibrary> {
[00:09:14] 
[00:09:14] error: method is never used: `symbol`
[00:09:14]   --> src/libstd/sys/windows/dynamic_lib.rs:27:5
[00:09:14]    |
[00:09:14]    |
[00:09:14] 27 |     pub fn symbol(&self, symbol: &str) -> io::Result<usize> {
[00:09:14] 
[00:09:14] error: aborting due to 15 previous errors
[00:09:14] 
[00:09:14] error: Could not compile `std`.
---
travis_time:end:0431abee:start=1557958440310103761,finish=1557958440316332526,duration=6228765
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26b50138
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14964528
travis_time:start:14964528
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:24a5ba00
$ dmesg | grep -i kill
