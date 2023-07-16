plain
travis_time:end:014a04c4:start=1543323024867771650,finish=1543323025931789515,duration=1064017865
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:06:35] configure: build.locked-deps    := True
[00:06:35] configure: llvm.ccache          := sccache
[00:06:35] configure: build.cargo-native-static := True
[00:06:35] configure: dist.missing-tools   := True
[00:06:35] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:06:35] configure: writing `config.toml` in current directory
[00:06:35] configure: 
[00:06:35] configure: run `python /checkout/x.py --help`
[00:06:35] configure: 
---
[00:08:02]     Checking libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:08:02]     Checking alloc v0.0.0 (/checkout/src/liballoc)
[00:08:02]     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:08:06]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:08:08] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:08]    |
[00:08:08]    |
[00:08:08] 33 | use mem::{self, MaybeUninit};
[00:08:08]    |
[00:08:08]    = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:08] 
[00:08:08] 
[00:08:08] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:08]     |
[00:08:08]     |
[00:08:08] 160 | pub struct ReentrantMutex { inner: MaybeUninit<UnsafeCell<c::CRITICAL_SECTION>> }
[00:08:08]     |
[00:08:08]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:08] 
[00:08:08] 
[00:08:08] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:08]     |
[00:08:08]     |
[00:08:08] 167 |         ReentrantMutex { inner: MaybeUninit::uninitialized() }
[00:08:08]     |
[00:08:08]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:08] 
[00:08:08] 
[00:08:10] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:10]     |
[00:08:10]     |
[00:08:10] 171 |         c::InitializeCriticalSection(self.inner.get_ref().get());
[00:08:10]     |
[00:08:10]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:10] 
[00:08:10] 
[00:08:10] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:10]     |
[00:08:10]     |
[00:08:10] 175 |         c::EnterCriticalSection(self.inner.get_ref().get());
[00:08:10]     |
[00:08:10]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:10] 
[00:08:10] 
[00:08:10] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:10]     |
[00:08:10]     |
[00:08:10] 180 |         c::TryEnterCriticalSection(self.inner.get_ref().get()) != 0
[00:08:10]     |
[00:08:10]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:10] 
[00:08:10] 
[00:08:10] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:10]     |
[00:08:10]     |
[00:08:10] 184 |         c::LeaveCriticalSection(self.inner.get_ref().get());
[00:08:10]     |
[00:08:10]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:10] 
[00:08:10] 
[00:08:10] error[E0658]: use of unstable library feature 'maybe_uninit' (see issue #53491)
[00:08:10]     |
[00:08:10]     |
[00:08:10] 188 |         c::DeleteCriticalSection(self.inner.get_ref().get());
[00:08:10]     |
[00:08:10]     = help: add #![feature(maybe_uninit)] to the crate attributes to enable
[00:08:10] 
[00:08:10] error: aborting due to 8 previous errors
[00:08:10] error: aborting due to 8 previous errors
[00:08:10] 
[00:08:10] For more information about this error, try `rustc --explain E0658`.
[00:08:10] error: Could not compile `std`.
[00:08:10] 
[00:08:10] To learn more, run the command again with --verbose.
[00:08:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:08:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
[00:08:10] Build completed unsuccessfully in 0:00:30
travis_time:end:10036f8c:start=1543323034683017313,finish=1543323526383134731,duration=491700117418
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:0712dff0:start=1543323526818164247,finish=1543323526825760020,duration=7595773
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1fc64d9c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00e32abe
travis_time:start:00e32abe
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3f5f8e7c
$ dmesg | grep -i kill
