plain
travis_time:end:12c15103:start=1543314315747488617,finish=1543314318036583390,duration=2289094773
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:06:49] configure: build.locked-deps    := True
[00:06:49] configure: llvm.ccache          := sccache
[00:06:49] configure: build.cargo-native-static := True
[00:06:49] configure: dist.missing-tools   := True
[00:06:49] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:06:49] configure: writing `config.toml` in current directory
[00:06:49] configure: 
[00:06:49] configure: run `python /checkout/x.py --help`
[00:06:49] configure: 
---
[00:08:15]     Checking libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:08:15]     Checking alloc v0.0.0 (/checkout/src/liballoc)
[00:08:15]     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:08:19]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:08:20] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:20]    |
[00:08:20]    |
[00:08:20] 33 | use mem::{self, MaybeUninit};
[00:08:20]    |
[00:08:20]    = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:20] 
[00:08:20] 
[00:08:20] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:20]     |
[00:08:20]     |
[00:08:20] 160 | pub struct ReentrantMutex { inner: MaybeUninit<UnsafeCell<c::CRITICAL_SECTION>> }
[00:08:20]     |
[00:08:20]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:20] 
[00:08:20] 
[00:08:20] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:20]     |
[00:08:20] 167 |         MaybeUninit::uninitialized()
[00:08:20]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:08:20]     |
[00:08:20]     |
[00:08:20]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:20] 
[00:08:22] error[E0308]: mismatched types
[00:08:22]    --> src/libstd/sys/windows/mutex.rs:167:9
[00:08:22]     |
[00:08:22] 166 |     pub fn uninitialized() -> ReentrantMutex {
[00:08:22]     |                               -------------- expected `sys::windows::mutex::ReentrantMutex` because of return type
[00:08:22] 167 |         MaybeUninit::uninitialized()
[00:08:22]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `sys::windows::mutex::ReentrantMutex`, found union `core::mem::MaybeUninit`
[00:08:22]     |
[00:08:22]     = note: expected type `sys::windows::mutex::ReentrantMutex`
[00:08:22]                found type `core::mem::MaybeUninit<_>`
[00:08:22] 
[00:08:22] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:22]     |
[00:08:22]     |
[00:08:22] 171 |         c::InitializeCriticalSection(self.inner.get_ref().get());
[00:08:22]     |
[00:08:22]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:22] 
[00:08:22] 
[00:08:22] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:22]     |
[00:08:22]     |
[00:08:22] 175 |         c::EnterCriticalSection(self.inner.get_ref().get());
[00:08:22]     |
[00:08:22]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:22] 
[00:08:22] 
[00:08:22] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:22]     |
[00:08:22]     |
[00:08:22] 180 |         c::TryEnterCriticalSection(self.inner.get_ref().get()) != 0
[00:08:22]     |
[00:08:22]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:22] 
[00:08:22] 
[00:08:22] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:22]     |
[00:08:22]     |
[00:08:22] 184 |         c::LeaveCriticalSection(self.inner.get_ref().get());
[00:08:22]     |
[00:08:22]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:22] 
[00:08:22] 
[00:08:22] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:22]     |
[00:08:22]     |
[00:08:22] 188 |         c::DeleteCriticalSection(self.inner.get_ref().get());
[00:08:22]     |
[00:08:22]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:22] 
[00:08:23] error: aborting due to 9 previous errors
[00:08:23] error: aborting due to 9 previous errors
[00:08:23] 
[00:08:23] Some errors occurred: E0308, E0658.
[00:08:23] For more information about an error, try `rustc --explain E0308`.
[00:08:23] error: Could not compile `std`.
[00:08:23] 
[00:08:23] To learn more, run the command again with --verbose.
[00:08:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:08:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
[00:08:23] Build completed unsuccessfully in 0:00:29
travis_time:end:2c00311c:start=1543314326974359037,finish=1543314830331025333,duration=503356666296
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:3330f676:start=1543314830767845251,finish=1543314830775483157,duration=7637906
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:092d0a8c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dd3b668
travis_time:start:0dd3b668
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05d49590
$ dmesg | grep -i kill
