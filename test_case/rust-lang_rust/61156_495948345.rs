plain
travis_time:end:1bb75eec:start=1558817823782868869,finish=1558817912698619195,duration=88915750326
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:13]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:05:18]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:05:18]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:05:18]    Compiling hashbrown v0.3.0
[00:05:21] error[E0390]: only a single inherent implementation marked with `#[lang = "slice"]` is allowed for the `[T]` primitive
[00:05:21]      |
[00:05:21]      |
[00:05:21] 1311 | / impl<S: Borrow<CStr>> [S] {
[00:05:21] 1312 | |     fn concat(&self) -> CString {
[00:05:21] 1313 | |         self.join(unsafe { CStr::from_bytes_with_nul_unchecked(&[0]) })
[00:05:21] ...    |
[00:05:21] 1333 | |     }
[00:05:21] 1334 | | }
[00:05:21]      | |_^
[00:05:21]      | |_^
[00:05:21]      |
[00:05:21] help: consider using a trait to implement these methods
[00:05:21]     --> src/libstd/ffi/c_str.rs:1311:1
[00:05:21]      |
[00:05:21] 1311 | / impl<S: Borrow<CStr>> [S] {
[00:05:21] 1312 | |     fn concat(&self) -> CString {
[00:05:21] 1313 | |         self.join(unsafe { CStr::from_bytes_with_nul_unchecked(&[0]) })
[00:05:21] ...    |
[00:05:21] 1333 | |     }
[00:05:21] 1334 | | }
[00:05:21]      | |_^
[00:05:21]      | |_^
[00:05:21] 
[00:05:21] error[E0390]: only a single inherent implementation marked with `#[lang = "slice"]` is allowed for the `[T]` primitive
[00:05:21]     |
[00:05:21]     |
[00:05:21] 974 | / impl<S: Borrow<OsStr>> [S] {
[00:05:21] 975 | |     fn concat(&self) -> OsString {
[00:05:21] 976 | |         self.join("".as_ref())
[00:05:21] ...   |
[00:05:21] 996 | |     }
[00:05:21] 997 | | }
[00:05:21]     | |_^
[00:05:21]     | |_^
[00:05:21]     |
[00:05:21] help: consider using a trait to implement these methods
[00:05:21]    --> src/libstd/ffi/os_str.rs:974:1
[00:05:21]     |
[00:05:21] 974 | / impl<S: Borrow<OsStr>> [S] {
[00:05:21] 975 | |     fn concat(&self) -> OsString {
[00:05:21] 976 | |         self.join("".as_ref())
[00:05:21] ...   |
[00:05:21] 996 | |     }
[00:05:21] 997 | | }
[00:05:21]     | |_^
---
travis_time:end:29cd0f00:start=1558818244513347713,finish=1558818244520149494,duration=6801781
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00237040
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19aa2472
travis_time:start:19aa2472
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04032304
$ dmesg | grep -i kill
