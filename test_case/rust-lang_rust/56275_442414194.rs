plain
travis_time:end:3182576f:start=1543397853918340421,finish=1543397938976201507,duration=85057861086
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:06:57] configure: build.locked-deps    := True
[00:06:57] configure: llvm.ccache          := sccache
[00:06:57] configure: build.cargo-native-static := True
[00:06:57] configure: dist.missing-tools   := True
[00:06:57] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:06:57] configure: writing `config.toml` in current directory
[00:06:57] configure: 
[00:06:57] configure: run `python /checkout/x.py --help`
[00:06:57] configure: 
---
[00:08:33]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:08:37] error[E0308]: mismatched types
[00:08:37]    --> src/libstd/sys/windows/mutex.rs:177:33
[00:08:37]     |
[00:08:37] 177 |         c::EnterCriticalSection((&mut *self.inner.get()).get_ref());
[00:08:37]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
[00:08:37]     |
[00:08:37]     = note: expected type `*mut sys::windows::c::CRITICAL_SECTION`
[00:08:37]                found type `&sys::windows::c::CRITICAL_SECTION`
[00:08:37] error[E0308]: mismatched types
[00:08:37]    --> src/libstd/sys/windows/mutex.rs:184:36
[00:08:37]     |
[00:08:37]     |
[00:08:37] 184 |         c::TryEnterCriticalSection((&mut *self.inner.get()).get_ref()) != 0
[00:08:37]     |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
[00:08:37]     |
[00:08:37]     = note: expected type `*mut sys::windows::c::CRITICAL_SECTION`
[00:08:37]                found type `&sys::windows::c::CRITICAL_SECTION`
[00:08:37] error[E0308]: mismatched types
[00:08:37]    --> src/libstd/sys/windows/mutex.rs:190:33
[00:08:37]     |
[00:08:37]     |
[00:08:37] 190 |         c::LeaveCriticalSection((&mut *self.inner.get()).get_ref());
[00:08:37]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
[00:08:37]     |
[00:08:37]     = note: expected type `*mut sys::windows::c::CRITICAL_SECTION`
[00:08:37]                found type `&sys::windows::c::CRITICAL_SECTION`
[00:08:37] error[E0308]: mismatched types
[00:08:37]    --> src/libstd/sys/windows/mutex.rs:196:34
[00:08:37]     |
[00:08:37]     |
[00:08:37] 196 |         c::DeleteCriticalSection((&mut *self.inner.get()).get_ref());
[00:08:37]     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
[00:08:37]     |
[00:08:37]     = note: expected type `*mut sys::windows::c::CRITICAL_SECTION`
[00:08:37]                found type `&sys::windows::c::CRITICAL_SECTION`
[00:08:37] error: aborting due to 4 previous errors
[00:08:37] 
[00:08:37] For more information about this error, try `rustc --explain E0308`.
[00:08:37] error: Could not compile `std`.
[00:08:37] error: Could not compile `std`.
[00:08:37] 
[00:08:37] To learn more, run the command again with --verbose.
[00:08:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:08:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
[00:08:37] Build completed unsuccessfully in 0:00:31
travis_time:end:0537a536:start=1543397948085105501,finish=1543398465822822309,duration=517737716808
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:13fc6046:start=1543398466291998481,finish=1543398466300421667,duration=8423186
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03b8a4c6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:279687e4
travis_time:start:279687e4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a5cb638
$ dmesg | grep -i kill
