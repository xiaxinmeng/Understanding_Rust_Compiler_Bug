
Updating only changed submodules
  Submodules updated in 0.01 seconds
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.08s
Build completed successfully in 0:00:00
Updating only changed submodules
  Submodules updated in 0.01 seconds
extracting /.../rust/build/cache/2021-11-30/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz, rustc-beta-x86_64-unknown-linux-gnu.tar.xz, cargo-beta-x86_64-unknown-linux-gnu.tar.xz, rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz, rust-dev-nightly-x86_64-unknown-linux-gnu.tar.xz
info: you seem to be using Nix. Attempting to patch /.../rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo, rustc, rustdoc, libstd-32f1f2d0c3a5e2ad.so, librustc_driver-6d26630eb1684640.so, libtest-5d4b7321cb221336.so, libLLVM-13-rust-1.58.0-beta.so, rustfmt, cargo-fmt, llvm-config, FileCheck, libLLVM-13-rust-1.60.0-nightly.so
Building rustbuild
   Compiling std v0.0.0 (/.../rust/library/std)
    Finished release [optimized] target(s) in 17.13s
     Running unittests (build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/std-75bdc7c332410008)

running 916 tests
........................................................................thread '.<unnamed>' panicked at 'explicit panic', library/std/src/io/buffered/tests.rs:495:13
........................... 300/916
...........................................thread '.<unnamed>' panicked at 'explicit panic', library/std/src/io/stdio/tests.rs:37:9.
....................................................... 400/916
...........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/sync_tests.rs:346:28
....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/sync_tests.rs:383:27
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: SendError { .. }', library/std/src/sync/mpsc/sync_tests.rs:371:24
......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/sync_tests.rs:250:19
..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/tests.rs:322:28
....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/tests.rs:359:27
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: SendError { .. }', library/std/src/sync/mpsc/tests.rs:347:24
. 700/916
.......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/tests.rs:241:19
.....................................thread '<unnamed>' panicked at 'explicit panic', library/std/src/sync/mutex/tests.rs:160:9
..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', library/std/src/sync/mutex/tests.rs:114:9
...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', library/std/src/sync/mutex/tests.rs:90:9
.thread '<unnamed>' panicked at 'explicit panic', library/std/src/sync/mutex/tests.rs:221:9
...thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', library/std/src/sync/mutex/tests.rs:184:9
..........thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', library/std/src/sync/rwlock/tests.rs:238:9
...thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', library/std/src/sync/rwlock/tests.rs:214:9
..thread '<unnamed>' panicked at 'explicit panic', library/std/src/sync/rwlock/tests.rs:150:9
.thread '<unnamed>' panicked at 'explicit panic', library/std/src/sync/rwlock/tests.rs:78:9
.thread '<unnamed>' panicked at 'explicit panic', library/std/src/sync/rwlock/tests.rs:90:9
.thread '<unnamed>' panicked at 'explicit panic', library/std/src/sync/rwlock/tests.rs:52:9
.thread '<unnamed>' panicked at 'explicit panic', library/std/src/sync/rwlock/tests.rs:65:9
...............panicked at 'crash now!', library/std/src/sys/unix/process/process_unix/tests.rs:46:27
panicked after panic::always_abort(), aborting.
............. 800/916
...................................................................thread '<unnamed>' panicked at 'explicit panic.', library/std/src/thread/tests.rs:86:33
...........thread '<unnamed>' panicked at 'Box<dyn Any>', library/std/src/thread/tests.rs:221:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
.thread '<unnamed>' panicked at 'owned string', library/std/src/thread/tests.rs:205:9
.thread '<unnamed>' panicked at 'Box<dyn Any>', library/std/src/thread/tests.rs:240:33
.thread '<unnamed>' panicked at 'static string', library/std/src/thread/tests.rs:189:9
.................. 900/916
test result: ok. 916 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.24s

     Running tests/env.rs (build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/env-dc89ea34df61ec2e)

running 7 tests
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/run-time-detect.rs (build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/run_time_detect-8dc918f91e61c122)

running 1 test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/thread.rs (build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/thread-754448f62787afc1)

running 1 test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

     Running unittests (build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/stdbenches-70cb4a8bd7a65d77)

running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests std

running 1204 tests
test result: ok. 1183 passed; 0 failed; 21 ignored; 0 measured; 0 filtered out; finished in 32.99s

	finished in 60.905 seconds
Build completed successfully in 0:01:52
