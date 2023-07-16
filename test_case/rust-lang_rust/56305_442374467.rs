plain
travis_time:end:2cb0c3cb:start=1543390232637230383,finish=1543390233641299972,duration=1004069589
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[01:38:47] failures:
[01:38:47] 
[01:38:47] ---- [ui] run-pass-fullmir/async-fn.rs stdout ----
[01:38:47] normalized stderr:
[01:38:47] error[E0080]: constant evaluation error: Stopping looking for borrow being accessed (Shr(None)) because of barrier (765)
[01:38:47]     --> /checkout/src/libcore/ptr.rs:2928:9
[01:38:47]      |
[01:38:47] 2928 |         &mut *self.as_ptr()
[01:38:47]      |         ^^^^^^^^^^^^^^^^^^^ Stopping looking for borrow being accessed (Shr(None)) because of barrier (765)
[01:38:47]      |
[01:38:47]      = note: inside call to `<std::ptr::NonNull<T>><std::task::LocalWaker>::as_mut` at /checkout/src/libstd/future.rs:101:16
[01:38:47]      = note: inside call to `std::future::get_task_waker::<[closure@DefId(1/1:1773 ~ std[de45]::future[0]::poll_with_tls_waker[0]::{{closure}}[0]) 0:std::pin::Pin<&mut std::future::GenFuture<[static generator@$DIR/async-fn.rs:15:26: 15:37 y:&&u32, z:&&u32 {}]>>], std::task::Poll<u32>>` at /checkout/src/libstd/future.rs:110:5
[01:38:47] note: inside call to `std::future::poll_with_tls_waker::<std::future::GenFuture<[static generator@$DIR/async-fn.rs:15:26: 15:37 y:&&u32, z:&&u32 {}]>>` at <::std::macros::await macros>:4:49
[01:38:47]     --> $DIR/async-fn.rs:15:13
[01:38:47]      |
[01:38:47] 15   |     let y = await!(async { *y + *z });
[01:38:47]      = note: inside call to closure at /checkout/src/libstd/future.rs:46:46
[01:38:47]      = note: inside call to closure at /checkout/src/libstd/future.rs:77:5
[01:38:47]      = note: inside call to closure at /checkout/src/libstd/future.rs:77:5
[01:38:47]      = note: inside call to `std::future::set_task_waker::<[closure@DefId(1/1:1762 ~ std[de45]::future[0]::{{impl}}[1]::poll[0]::{{closure}}[0]) 0:std::pin::Pin<&mut std::future::GenFuture<[static generator@$DIR/async-fn.rs:11:42: 19:2 y:u32, x:&u32 for<'r, 's, 't0, 't1, 't2, 't3> {u32, &'r u32, &'s u32, impl std::future::Future, ()}]>>], std::task::Poll<u32>>` at /checkout/src/libstd/future.rs:46:9
[01:38:47] note: inside call to `<std::future::GenFuture<T> as std::future::Future><[static generator@$DIR/async-fn.rs:11:42: 19:2 y:u32, x:&u32 for<'r, 's, 't0, 't1, 't2, 't3> {u32, &'r u32, &'s u32, impl std::future::Future, ()}]>::poll` at $DIR/async-fn.rs:34:16
[01:38:47]     --> $DIR/async-fn.rs:34:16
[01:38:47]      |
[01:38:47] 34   |     assert_eq!(unsafe { Pin::new_unchecked(&mut fut) }.poll(&lw), Poll::Ready(31));
[01:38:47]      |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:38:47]      = note: inside call to `main` at /checkout/src/libstd/rt.rs:74:34
[01:38:47]      = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:136:5
[01:38:47]      = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:136:5
[01:38:47]      = note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1/1:1914 ~ std[de45]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:59:13
[01:38:47]      = note: inside call to closure at /checkout/src/libstd/panicking.rs:310:40
[01:38:47]      = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1/1:1913 ~ std[de45]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:306:5
[01:38:47]      = note: inside call to `std::panicking::try::<i32, [closure@DefId(1/1:1913 ~ std[de45]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:398:9
[01:38:47]      = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1/1:1913 ~ std[de45]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:58:25
[01:38:47]      = note: inside call to `std::rt::lang_start_internal` at /checkout/src/libstd/rt.rs:74:5
[01:38:47]      = note: inside call to `std::rt::lang_start::<()>`
[01:38:47] 
[01:38:47] error: aborting due to previous error
[01:38:47] 
[01:38:47] For more information about this error, try `rustc --explain E0080`.
[01:38:47] For more information about this error, try `rustc --explain E0080`.
[01:38:47] 
[01:38:47] 
[01:38:47] expected stderr:
[01:38:47] 
[01:38:47] 
[01:38:47] diff of stderr:
[01:38:47] 
[01:38:47] +error[E0080]: constant evaluation error: Stopping looking for borrow being accessed (Shr(None)) because of barrier (765)
[01:38:47] +    --> /checkout/src/libcore/ptr.rs:2928:9
[01:38:47] +     |
[01:38:47] +2928 |         &mut *self.as_ptr()
[01:38:47] +     |         ^^^^^^^^^^^^^^^^^^^ Stopping looking for borrow being accessed (Shr(None)) because of barrier (765)
[01:38:47] +     |
[01:38:47] +     = note: inside call to `<std::ptr::NonNull<T>><std::task::LocalWaker>::as_mut` at /checkout/src/libstd/future.rs:101:16
[01:38:47] +     = note: inside call to `std::future::get_task_waker::<[closure@DefId(1/1:1773 ~ std[de45]::future[0]::poll_with_tls_waker[0]::{{closure}}[0]) 0:std::pin::Pin<&mut std::future::GenFuture<[static generator@$DIR/async-fn.rs:15:26: 15:37 y:&&u32, z:&&u32 {}]>>], std::task::Poll<u32>>` at /checkout/src/libstd/future.rs:110:5
[01:38:47] +note: inside call to `std::future::poll_with_tls_waker::<std::future::GenFuture<[static generator@$DIR/async-fn.rs:15:26: 15:37 y:&&u32, z:&&u32 {}]>>` at <::std::macros::await macros>:4:49
[01:38:47] +    --> $DIR/async-fn.rs:15:13
[01:38:47] +     |
[01:38:47] +15   |     let y = await!(async { *y + *z });
[01:38:47] +     = note: inside call to closure at /checkout/src/libstd/future.rs:46:46
[01:38:47] +     = note: inside call to closure at /checkout/src/libstd/future.rs:77:5
[01:38:47] +     = note: inside call to closure at /checkout/src/libstd/future.rs:77:5
[01:38:47] +     = note: inside call to `std::future::set_task_waker::<[closure@DefId(1/1:1762 ~ std[de45]::future[0]::{{impl}}[1]::poll[0]::{{closure}}[0]) 0:std::pin::Pin<&mut std::future::GenFuture<[static generator@$DIR/async-fn.rs:11:42: 19:2 y:u32, x:&u32 for<'r, 's, 't0, 't1, 't2, 't3> {u32, &'r u32, &'s u32, impl std::future::Future, ()}]>>], std::task::Poll<u32>>` at /checkout/src/libstd/future.rs:46:9
[01:38:47] +note: inside call to `<std::future::GenFuture<T> as std::future::Future><[static generator@$DIR/async-fn.rs:11:42: 19:2 y:u32, x:&u32 for<'r, 's, 't0, 't1, 't2, 't3> {u32, &'r u32, &'s u32, impl std::future::Future, ()}]>::poll` at $DIR/async-fn.rs:34:16
[01:38:47] +    --> $DIR/async-fn.rs:34:16
[01:38:47] +     |
[01:38:47] +34   |     assert_eq!(unsafe { Pin::new_unchecked(&mut fut) }.poll(&lw), Poll::Ready(31));
[01:38:47] +     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:38:47] +     = note: inside call to `main` at /checkout/src/libstd/rt.rs:74:34
[01:38:47] +     = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:136:5
[01:38:47] +     = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:136:5
[01:38:47] +     = note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1/1:1914 ~ std[de45]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:59:13
[01:38:47] +     = note: inside call to closure at /checkout/src/libstd/panicking.rs:310:40
[01:38:47] +     = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1/1:1913 ~ std[de45]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:306:5
[01:38:47] +     = note: inside call to `std::panicking::try::<i32, [closure@DefId(1/1:1913 ~ std[de45]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:398:9
[01:38:47] +     = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1/1:1913 ~ std[de45]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:58:25
[01:38:47] +     = note: inside call to `std::rt::lang_start_internal` at /checkout/src/libstd/rt.rs:74:5
[01:38:47] +     = note: inside call to `std::rt::lang_start::<()>`
[01:38:47] +
[01:38:47] +error: aborting due to previous error
[01:38:47] +
[01:38:47] +For more information about this error, try `rustc --explain E0080`.
[01:38:47] +For more information about this error, try `rustc --explain E0080`.
[01:38:47] +
[01:38:47] 
[01:38:47] The actual stderr differed from the expected stderr.
[01:38:47] Actual stderr saved to /tmp/compiletestHcnrJs/async-fn.stderr
[01:38:47] To update references, run this command from build directory:
[01:38:47] tests/run-pass-fullmir/update-references.sh '/tmp/compiletestHcnrJs' 'async-fn.rs'
[01:38:47] error: 1 errors occurred comparing output.
[01:38:47] status: exit code: 1
[01:38:47] status: exit code: 1
[01:38:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass-fullmir/async-fn.rs" "-L" "/tmp/compiletestHcnrJs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestHcnrJs/async-fn.stage-id" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestHcnrJs/async-fn.stage-id.aux" "-A" "unused"
[01:38:47] ------------------------------------------
[01:38:47] 
[01:38:47] ------------------------------------------
[01:38:47] stderr:
[01:38:47] stderr:
[01:38:47] ------------------------------------------
[01:38:47] error[E0080]: constant evaluation error: Stopping looking for borrow being accessed (Shr(None)) because of barrier (765)
[01:38:47]     --> /checkout/src/libcore/ptr.rs:2928:9
[01:38:47]      |
[01:38:47] 2928 |         &mut *self.as_ptr()
[01:38:47]      |         ^^^^^^^^^^^^^^^^^^^ Stopping looking for borrow being accessed (Shr(None)) because of barrier (765)
[01:38:47]      |
[01:38:47]      = note: inside call to `<std::ptr::NonNull<T>><std::task::LocalWaker>::as_mut` at /checkout/src/libstd/future.rs:101:16
[01:38:47]      = note: inside call to `std::future::get_task_waker::<[closure@DefId(1/1:1773 ~ std[de45]::future[0]::poll_with_tls_waker[0]::{{closure}}[0]) 0:std::pin::Pin<&mut std::future::GenFuture<[static generator@tests/run-pass-fullmir/async-fn.rs:15:26: 15:37 y:&&u32, z:&&u32 {}]>>], std::task::Poll<u32>>` at /checkout/src/libstd/future.rs:110:5
[01:38:47] note: inside call to `std::future::poll_with_tls_waker::<std::future::GenFuture<[static generator@tests/run-pass-fullmir/async-fn.rs:15:26: 15:37 y:&&u32, z:&&u32 {}]>>` at <::std::macros::await macros>:4:49
[01:38:47]     --> tests/run-pass-fullmir/async-fn.rs:15:13
[01:38:47]      |
[01:38:47] 15   |     let y = await!(async { *y + *z });
[01:38:47]      = note: inside call to closure at /checkout/src/libstd/future.rs:46:46
[01:38:47]      = note: inside call to closure at /checkout/src/libstd/future.rs:77:5
[01:38:47]      = note: inside call to closure at /checkout/src/libstd/future.rs:77:5
[01:38:47]      = note: inside call to `std::future::set_task_waker::<[closure@DefId(1/1:1762 ~ std[de45]::future[0]::{{impl}}[1]::poll[0]::{{closure}}[0]) 0:std::pin::Pin<&mut std::future::GenFuture<[static generator@tests/run-pass-fullmir/async-fn.rs:11:42: 19:2 y:u32, x:&u32 for<'r, 's, 't0, 't1, 't2, 't3> {u32, &'r u32, &'s u32, impl std::future::Future, ()}]>>], std::task::Poll<u32>>` at /checkout/src/libstd/future.rs:46:9
[01:38:47] note: inside call to `<std::future::GenFuture<T> as std::future::Future><[static generator@tests/run-pass-fullmir/async-fn.rs:11:42: 19:2 y:u32, x:&u32 for<'r, 's, 't0, 't1, 't2, 't3> {u32, &'r u32, &'s u32, impl std::future::Future, ()}]>::poll` at tests/run-pass-fullmir/async-fn.rs:34:16
[01:38:47]     --> tests/run-pass-fullmir/async-fn.rs:34:16
[01:38:47]      |
[01:38:47] 34   |     assert_eq!(unsafe { Pin::new_unchecked(&mut fut) }.poll(&lw), Poll::Ready(31));
[01:38:47]      |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:38:47]      = note: inside call to `main` at /checkout/src/libstd/rt.rs:74:34
[01:38:47]      = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:136:5
[01:38:47]      = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:136:5
[01:38:47]      = note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1/1:1914 ~ std[de45]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:59:13
[01:38:47]      = note: inside call to closure at /checkout/src/libstd/panicking.rs:310:40
[01:38:47]      = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1/1:1913 ~ std[de45]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:306:5
[01:38:47]      = note: inside call to `std::panicking::try::<i32, [closure@DefId(1/1:1913 ~ std[de45]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:398:9
[01:38:47]      = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1/1:1913 ~ std[de45]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:58:25
[01:38:47]      = note: inside call to `std::rt::lang_start_internal` at /checkout/src/libstd/rt.rs:74:5
[01:38:47]      = note: inside call to `std::rt::lang_start::<()>`
[01:38:47] 
[01:38:47] error: aborting due to previous error
[01:38:47] 
[01:38:47] For more information about this error, try `rustc --explain E0080`.
---
[01:38:47] Verifying status of clippy-driver...
[01:38:47] Verifying status of miri...
[01:38:47] This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
[01:38:47] 
[01:38:47] ⚠️ We detected that this PR updated 'miri', but its tests failed.
[01:38:47] 
[01:38:47] If you do intend to update 'miri', please check the error messages above and
[01:38:47] commit another update.
[01:38:47] 
[01:38:47] If you do NOT intend to update 'miri', please ensure you did not accidentally
[01:38:47] change the submodule at 'src/tools/miri'. You may ask your reviewer for the
[01:38:47] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:1b7c46df
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 28 09:09:30 UTC 2018
---
travis_time:end:121843dd:start=1543396171259562376,finish=1543396171265157134,duration=5594758
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12088b55
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09a1f478
travis_time:start:09a1f478
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02d0b7ea
$ dmesg | grep -i kill
