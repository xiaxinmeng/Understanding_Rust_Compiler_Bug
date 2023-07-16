plain
[01:12:34] [RUSTC-TIMING] panic_unwind test:false 0.280
[01:12:34] warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`
[01:12:34] 
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]   --> src/libstd/sys/sgx/abi/tls.rs:87:38
[01:12:35]    |
[01:12:35] 87 |     pub unsafe fn activate(&self) -> ActiveTls {
[01:12:35] 
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/abi/tls.rs:144:31
[01:12:35]     |
[01:12:35] 144 |         pub fn iter(&self) -> SyncBitsetIter {
[01:12:35] 
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/abi/usercalls/alloc.rs:432:27
[01:12:35] 432 |     pub fn iter(&self) -> Iter<T>
[01:12:35]     |                           ^^^^^^^ help: indicate the anonymous lifetime: `Iter<'_, T>`
[01:12:35] 
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/abi/usercalls/alloc.rs:441:35
[01:12:35]     |
[01:12:35] 441 |     pub fn iter_mut(&mut self) -> IterMut<T>
[01:12:35]     |                                   ^^^^^^^^^^ help: indicate the anonymous lifetime: `IterMut<'_, T>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/waitqueue.rs:143:31
[01:12:35]     |
[01:12:35] 143 |     pub fn wait<T>(mut guard: SpinMutexGuard<WaitVariable<T>>) {
[01:12:35]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SpinMutexGuard<'_, WaitVariable<T>>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/waitqueue.rs:165:37
[01:12:35]     |
[01:12:35] 165 |     pub fn notify_one<T>(mut guard: SpinMutexGuard<WaitVariable<T>>)
[01:12:35]     |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SpinMutexGuard<'_, WaitVariable<T>>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/waitqueue.rs:166:19
[01:12:35]     |
[01:12:35] 166 |         -> Result<WaitGuard<T>, SpinMutexGuard<WaitVariable<T>>>
[01:12:35]     |                   ^^^^^^^^^^^^ help: indicate the anonymous lifetime: `WaitGuard<'_, T>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/waitqueue.rs:166:33
[01:12:35]     |
[01:12:35] 166 |         -> Result<WaitGuard<T>, SpinMutexGuard<WaitVariable<T>>>
[01:12:35]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SpinMutexGuard<'_, WaitVariable<T>>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/waitqueue.rs:189:37
[01:12:35]     |
[01:12:35] 189 |     pub fn notify_all<T>(mut guard: SpinMutexGuard<WaitVariable<T>>)
[01:12:35]     |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SpinMutexGuard<'_, WaitVariable<T>>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/waitqueue.rs:190:19
[01:12:35]     |
[01:12:35] 190 |         -> Result<WaitGuard<T>, SpinMutexGuard<WaitVariable<T>>>
[01:12:35]     |                   ^^^^^^^^^^^^ help: indicate the anonymous lifetime: `WaitGuard<'_, T>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/waitqueue.rs:190:33
[01:12:35]     |
[01:12:35] 190 |         -> Result<WaitGuard<T>, SpinMutexGuard<WaitVariable<T>>>
[01:12:35]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SpinMutexGuard<'_, WaitVariable<T>>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/waitqueue.rs:436:31
[01:12:35]     |
[01:12:35] 436 |         pub fn lock(&self) -> SpinMutexGuard<T> {
[01:12:35]     |                               ^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SpinMutexGuard<'_, T>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/waitqueue.rs:448:42
[01:12:35]     |
[01:12:35] 448 |         pub fn try_lock(&self) -> Option<SpinMutexGuard<T>> {
[01:12:35]     |                                          ^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SpinMutexGuard<'_, T>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]   --> src/libstd/sys/sgx/backtrace.rs:35:66
[01:12:35]    |
[01:12:35] 35 |         unsafe { uw::_Unwind_Backtrace(trace_fn, &mut cx as *mut Context as *mut libc::c_void) };
[01:12:35] 
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]   --> src/libstd/sys/sgx/net.rs:51:27
[01:12:35] 51 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[01:12:35]    |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[01:12:35] 
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/net.rs:216:27
[01:12:35] 216 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[01:12:35]     |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[01:12:35] 
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]   --> src/libstd/sys/sgx/rwlock.rs:96:21
[01:12:35]    |
[01:12:35] 96 |         mut rguard: SpinMutexGuard<WaitVariable<Option<NonZeroUsize>>>,
[01:12:35]    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SpinMutexGuard<'_, WaitVariable<Option<NonZeroUsize>>>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]   --> src/libstd/sys/sgx/rwlock.rs:97:17
[01:12:35]    |
[01:12:35] 97 |         wguard: SpinMutexGuard<WaitVariable<bool>>,
[01:12:35]    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SpinMutexGuard<'_, WaitVariable<bool>>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/rwlock.rs:123:17
[01:12:35]     |
[01:12:35] 123 |         rguard: SpinMutexGuard<WaitVariable<Option<NonZeroUsize>>>,
[01:12:35]     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SpinMutexGuard<'_, WaitVariable<Option<NonZeroUsize>>>`
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35] error: hidden lifetime parameters in types are deprecated
[01:12:35]    --> src/libstd/sys/sgx/rwlock.rs:124:17
[01:12:35]     |
[01:12:35] 124 |         wguard: SpinMutexGuard<WaitVariable<bool>>,
[01:12:35]     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SpinMutexGuard<'_, WaitVariable<bool>>`
[01:12:39] error: aborting due to 20 previous errors
[01:12:39] 
[01:12:39] [RUSTC-TIMING] std test:false 4.844
[01:12:39] error: Could not compile `std`.
---
travis_time:end:0a9062a8:start=1554012321647463069,finish=1554012321657640040,duration=10176971
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f90e6b6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13da91a8
travis_time:start:13da91a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2f996607
$ dmesg | grep -i kill
