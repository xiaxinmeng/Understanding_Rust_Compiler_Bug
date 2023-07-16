plain
.................................................................................................... 3100/3758
.................................................................................................... 3200/3758
.................................................................................................... 3300/3758
...............................................................................i.................... 3400/3758
...........................i............F....i..................F...i......................iF....... 3500/3758
...........F..i..................F...i.................F..........F.....i....................F.i.... 3600/3758
................F.i...............F......i..................F...i...................F............... 3700/3758
failures:

---- src/sync/atomic.rs - sync::atomic::AtomicBool::from_mut_slice (line 347) stdout ----
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
  --> src/sync/atomic.rs:355:11
   |
11 |         s.spawn(move |_| a[i].store(true, Ordering::Relaxed));
   |           ^^^^^ -------- takes 1 argument
   |           expected closure that takes 0 arguments

error: aborting due to previous error


For more information about this error, try `rustc --explain E0593`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI16::from_mut_slice (line 2229) stdout ----
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
  --> src/sync/atomic.rs:2237:11
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |           ^^^^^ -------- takes 1 argument
   |           expected closure that takes 0 arguments

error: aborting due to previous error


For more information about this error, try `rustc --explain E0593`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI32::from_mut_slice (line 2267) stdout ----
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
  --> src/sync/atomic.rs:2275:11
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |           ^^^^^ -------- takes 1 argument
   |           expected closure that takes 0 arguments

error: aborting due to previous error


For more information about this error, try `rustc --explain E0593`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI64::from_mut_slice (line 2305) stdout ----
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
  --> src/sync/atomic.rs:2313:11
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |           ^^^^^ -------- takes 1 argument
   |           expected closure that takes 0 arguments

error: aborting due to previous error


For more information about this error, try `rustc --explain E0593`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI8::from_mut_slice (line 2191) stdout ----
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
  --> src/sync/atomic.rs:2199:11
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |           ^^^^^ -------- takes 1 argument
   |           expected closure that takes 0 arguments

error: aborting due to previous error


For more information about this error, try `rustc --explain E0593`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicIsize::from_mut_slice (line 2426) stdout ----
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
  --> src/sync/atomic.rs:2434:11
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |           ^^^^^ -------- takes 1 argument
   |           expected closure that takes 0 arguments

error: aborting due to previous error


For more information about this error, try `rustc --explain E0593`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicPtr<T>::from_mut_slice (line 978) stdout ----
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
  --> src/sync/atomic.rs:987:11
   |
12 |         s.spawn(move |_| {
   |           ^^^^^ -------- takes 1 argument
   |           expected closure that takes 0 arguments

error: aborting due to previous error


For more information about this error, try `rustc --explain E0593`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU16::from_mut_slice (line 2248) stdout ----
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
  --> src/sync/atomic.rs:2256:11
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |           ^^^^^ -------- takes 1 argument
   |           expected closure that takes 0 arguments

error: aborting due to previous error


For more information about this error, try `rustc --explain E0593`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU32::from_mut_slice (line 2286) stdout ----
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
  --> src/sync/atomic.rs:2294:11
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |           ^^^^^ -------- takes 1 argument
   |           expected closure that takes 0 arguments

error: aborting due to previous error


For more information about this error, try `rustc --explain E0593`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU64::from_mut_slice (line 2324) stdout ----
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
  --> src/sync/atomic.rs:2332:11
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |           ^^^^^ -------- takes 1 argument
   |           expected closure that takes 0 arguments

error: aborting due to previous error


For more information about this error, try `rustc --explain E0593`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU8::from_mut_slice (line 2210) stdout ----
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
  --> src/sync/atomic.rs:2218:11
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |           ^^^^^ -------- takes 1 argument
   |           expected closure that takes 0 arguments

error: aborting due to previous error


For more information about this error, try `rustc --explain E0593`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicUsize::from_mut_slice (line 2426) stdout ----
error[E0593]: closure is expected to take 0 arguments, but it takes 1 argument
  --> src/sync/atomic.rs:2434:11
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |           ^^^^^ -------- takes 1 argument
   |           expected closure that takes 0 arguments

error: aborting due to previous error

