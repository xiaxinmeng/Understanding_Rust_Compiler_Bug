plain
travis_time:end:0356dca2:start=1543952325411240058,finish=1543952326592044831,duration=1180804773
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:03:19]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:26] warning: function cannot return without recursing
[00:03:26]     --> src/libcore/slice/mod.rs:2657:5
[00:03:26]      |
[00:03:26] 2657 |     unsafe fn get_unchecked(self, slice: &[T]) -> &[T] {
[00:03:26]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
[00:03:26] 2658 |         (*self.start()..=*self.end()).get_unchecked(slice)
[00:03:26]      |         -------------------------------------------------- recursive call site
[00:03:26]      = note: #[warn(unconditional_recursion)] on by default
[00:03:26]      = note: #[warn(unconditional_recursion)] on by default
[00:03:26]      = help: a `loop` may express intention better if this is on purpose
[00:03:26] warning: function cannot return without recursing
[00:03:26]     --> src/libcore/slice/mod.rs:2662:5
[00:03:26]      |
[00:03:26]      |
[00:03:26] 2662 |     unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] {
[00:03:26]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
[00:03:26] 2663 |         (*self.start()..=*self.end()).get_unchecked_mut(slice)
[00:03:26]      |         ------------------------------------------------------ recursive call site
[00:03:26]      |
[00:03:26]      = help: a `loop` may express intention better if this is on purpose
[00:03:26] warning: function cannot return without recursing
[00:03:26]     --> src/libcore/slice/mod.rs:2667:5
[00:03:26]      |
[00:03:26]      |
[00:03:26] 2667 |     fn index(self, slice: &[T]) -> &[T] {
[00:03:26]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
[00:03:26] 2668 |         if *self.end() == usize::max_value() { slice_index_overflow_fail(); }
[00:03:26] 2669 |         (*self.start()..=*self.end()).index(slice)
[00:03:26]      |         ------------------------------------------ recursive call site
[00:03:26]      |
[00:03:26]      = help: a `loop` may express intention better if this is on purpose
[00:03:26] warning: function cannot return without recursing
[00:03:26]     --> src/libcore/slice/mod.rs:2673:5
[00:03:26]      |
[00:03:26]      |
[00:03:26] 2673 |     fn index_mut(self, slice: &mut [T]) -> &mut [T] {
[00:03:26]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
[00:03:26] 2674 |         if *self.end() == usize::max_value() { slice_index_overflow_fail(); }
[00:03:26] 2675 |         (*self.start()..=*self.end()).index_mut(slice)
[00:03:26]      |         ---------------------------------------------- recursive call site
[00:03:26]      |
[00:03:26]      = help: a `loop` may express intention better if this is on purpose
[00:03:34]    Compiling libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:03:34]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:03:35]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:03:39]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
---
[00:19:00] Assembling stage1 compiler (x86_64-unknown-linux-gnu)
[00:19:00] travis_fold:start:stage1-std
travis_time:start:stage1-std
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:19:00] error: process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc -vV` (signal: 6, SIGABRT: process abort signal)
[00:19:00] 
[00:19:00] 
[00:19:00] thread 'main' has overflowed its stack
[00:19:00] fatal runtime error: stack overflow
[00:19:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:19:00] expected success, got: exit code: 101
[00:19:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:00] Build completed unsuccessfully in 0:15:54
[00:19:00] Build completed unsuccessfully in 0:15:54
[00:19:00] Makefile:28: recipe for target 'all' failed
[00:19:00] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0895b3de
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec  4 19:57:55 UTC 2018
---
travis_time:end:05e411e2:start=1543953476449658124,finish=1543953476455260810,duration=5602686
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04fb9b70
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.5320.!checkout!obj!build!x86_64-unknown-linux-gnu!stage1!bin!rustc
[New LWP 5320]
warning: Could not load shared library symbols for 7 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
