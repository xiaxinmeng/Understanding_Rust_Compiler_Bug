plain
.................................................................................................... 3100/3758
.................................................................................................... 3200/3758
.................................................................................................... 3300/3758
...............................................................................i.................... 3400/3758
...........................i.............F...i..................F...i......................iF....... 3500/3758
..........F...i..................F...i.....................F............iF.....................iF... 3600/3758
..............F...i......................iF.....................i.F..................F.............. 3700/3758
failures:

---- src/sync/atomic.rs - sync::atomic::AtomicBool::from_mut_slice (line 347) stdout ----
error[E0425]: cannot find value `s` in this scope
error[E0425]: cannot find value `s` in this scope
  --> src/sync/atomic.rs:355:9
   |
11 |         s.spawn(move |_| a[i].store(true, Ordering::Relaxed));
   |         ^ help: a local variable with a similar name exists: `a`
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
 --> src/sync/atomic.rs:353:1
  |
9 | std::thread::scope(|| {
9 | std::thread::scope(|| {
  | ^^^^^^^^^^^^^^^^^^ -- takes 0 arguments
  | |
  | expected closure that takes 1 argument
  |
help: consider changing the closure to take and ignore the expected argument
  |
9 | std::thread::scope(|_| {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0593.
Some errors have detailed explanations: E0425, E0593.
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI16::from_mut_slice (line 2229) stdout ----
error[E0425]: cannot find value `s` in this scope
  --> src/sync/atomic.rs:2237:9
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |         ^ help: a local variable with a similar name exists: `a`
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
 --> src/sync/atomic.rs:2235:1
  |
9 | std::thread::scope(|| {
9 | std::thread::scope(|| {
  | ^^^^^^^^^^^^^^^^^^ -- takes 0 arguments
  | |
  | expected closure that takes 1 argument
  |
help: consider changing the closure to take and ignore the expected argument
  |
9 | std::thread::scope(|_| {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0593.
Some errors have detailed explanations: E0425, E0593.
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI32::from_mut_slice (line 2267) stdout ----
error[E0425]: cannot find value `s` in this scope
  --> src/sync/atomic.rs:2275:9
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |         ^ help: a local variable with a similar name exists: `a`
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
 --> src/sync/atomic.rs:2273:1
  |
9 | std::thread::scope(|| {
9 | std::thread::scope(|| {
  | ^^^^^^^^^^^^^^^^^^ -- takes 0 arguments
  | |
  | expected closure that takes 1 argument
  |
help: consider changing the closure to take and ignore the expected argument
  |
9 | std::thread::scope(|_| {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0593.
Some errors have detailed explanations: E0425, E0593.
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI64::from_mut_slice (line 2305) stdout ----
error[E0425]: cannot find value `s` in this scope
  --> src/sync/atomic.rs:2313:9
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |         ^ help: a local variable with a similar name exists: `a`
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
 --> src/sync/atomic.rs:2311:1
  |
9 | std::thread::scope(|| {
9 | std::thread::scope(|| {
  | ^^^^^^^^^^^^^^^^^^ -- takes 0 arguments
  | |
  | expected closure that takes 1 argument
  |
help: consider changing the closure to take and ignore the expected argument
  |
9 | std::thread::scope(|_| {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0593.
Some errors have detailed explanations: E0425, E0593.
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicI8::from_mut_slice (line 2191) stdout ----
error[E0425]: cannot find value `s` in this scope
  --> src/sync/atomic.rs:2199:9
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |         ^ help: a local variable with a similar name exists: `a`
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
 --> src/sync/atomic.rs:2197:1
  |
9 | std::thread::scope(|| {
9 | std::thread::scope(|| {
  | ^^^^^^^^^^^^^^^^^^ -- takes 0 arguments
  | |
  | expected closure that takes 1 argument
  |
help: consider changing the closure to take and ignore the expected argument
  |
9 | std::thread::scope(|_| {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0593.
Some errors have detailed explanations: E0425, E0593.
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicIsize::from_mut_slice (line 2426) stdout ----
error[E0425]: cannot find value `s` in this scope
  --> src/sync/atomic.rs:2434:9
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |         ^ help: a local variable with a similar name exists: `a`
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
 --> src/sync/atomic.rs:2432:1
  |
9 | std::thread::scope(|| {
9 | std::thread::scope(|| {
  | ^^^^^^^^^^^^^^^^^^ -- takes 0 arguments
  | |
  | expected closure that takes 1 argument
  |
help: consider changing the closure to take and ignore the expected argument
  |
9 | std::thread::scope(|_| {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0593.
Some errors have detailed explanations: E0425, E0593.
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicPtr<T>::from_mut_slice (line 978) stdout ----
error[E0425]: cannot find value `s` in this scope
  --> src/sync/atomic.rs:987:9
   |
12 |         s.spawn(move || {
   |         ^ help: a local variable with a similar name exists: `a`
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
  --> src/sync/atomic.rs:985:1
   |
10 | std::thread::scope(|| {
10 | std::thread::scope(|| {
   | ^^^^^^^^^^^^^^^^^^ -- takes 0 arguments
   | |
   | expected closure that takes 1 argument
   |
help: consider changing the closure to take and ignore the expected argument
   |
10 | std::thread::scope(|_| {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0593.
Some errors have detailed explanations: E0425, E0593.
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU16::from_mut_slice (line 2248) stdout ----
error[E0425]: cannot find value `s` in this scope
  --> src/sync/atomic.rs:2256:9
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |         ^ help: a local variable with a similar name exists: `a`
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
 --> src/sync/atomic.rs:2254:1
  |
9 | std::thread::scope(|| {
9 | std::thread::scope(|| {
  | ^^^^^^^^^^^^^^^^^^ -- takes 0 arguments
  | |
  | expected closure that takes 1 argument
  |
help: consider changing the closure to take and ignore the expected argument
  |
9 | std::thread::scope(|_| {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0593.
Some errors have detailed explanations: E0425, E0593.
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU32::from_mut_slice (line 2286) stdout ----
error[E0425]: cannot find value `s` in this scope
  --> src/sync/atomic.rs:2294:9
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |         ^ help: a local variable with a similar name exists: `a`
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
 --> src/sync/atomic.rs:2292:1
  |
9 | std::thread::scope(|| {
9 | std::thread::scope(|| {
  | ^^^^^^^^^^^^^^^^^^ -- takes 0 arguments
  | |
  | expected closure that takes 1 argument
  |
help: consider changing the closure to take and ignore the expected argument
  |
9 | std::thread::scope(|_| {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0593.
Some errors have detailed explanations: E0425, E0593.
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU64::from_mut_slice (line 2324) stdout ----
error[E0425]: cannot find value `s` in this scope
  --> src/sync/atomic.rs:2332:9
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |         ^ help: a local variable with a similar name exists: `a`
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
 --> src/sync/atomic.rs:2330:1
  |
9 | std::thread::scope(|| {
9 | std::thread::scope(|| {
  | ^^^^^^^^^^^^^^^^^^ -- takes 0 arguments
  | |
  | expected closure that takes 1 argument
  |
help: consider changing the closure to take and ignore the expected argument
  |
9 | std::thread::scope(|_| {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0593.
Some errors have detailed explanations: E0425, E0593.
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicU8::from_mut_slice (line 2210) stdout ----
error[E0425]: cannot find value `s` in this scope
  --> src/sync/atomic.rs:2218:9
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |         ^ help: a local variable with a similar name exists: `a`
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
 --> src/sync/atomic.rs:2216:1
  |
9 | std::thread::scope(|| {
9 | std::thread::scope(|| {
  | ^^^^^^^^^^^^^^^^^^ -- takes 0 arguments
  | |
  | expected closure that takes 1 argument
  |
help: consider changing the closure to take and ignore the expected argument
  |
9 | std::thread::scope(|_| {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0593.
Some errors have detailed explanations: E0425, E0593.
For more information about an error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/sync/atomic.rs - sync::atomic::AtomicUsize::from_mut_slice (line 2426) stdout ----
error[E0425]: cannot find value `s` in this scope
  --> src/sync/atomic.rs:2434:9
   |
11 |         s.spawn(move |_| a[i].store(i as _, Ordering::Relaxed));
   |         ^ help: a local variable with a similar name exists: `a`
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
 --> src/sync/atomic.rs:2432:1
  |
9 | std::thread::scope(|| {
9 | std::thread::scope(|| {
  | ^^^^^^^^^^^^^^^^^^ -- takes 0 arguments
  | |
  | expected closure that takes 1 argument
  |
help: consider changing the closure to take and ignore the expected argument
  |
9 | std::thread::scope(|_| {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0593.
