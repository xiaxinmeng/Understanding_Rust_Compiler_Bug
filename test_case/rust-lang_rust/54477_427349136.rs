
% rustup override set nightly-2018-08-29-x86_64-unknown-linux-gnu
info: using existing install for 'nightly-2018-08-29-x86_64-unknown-linux-gnu'
info: override toolchain for '/home/pnkfelix/Dev/Mozilla/issue54477/collection' set to 'nightly-2018-08-29-x86_64-unknown-linux-gnu'

  nightly-2018-08-29-x86_64-unknown-linux-gnu unchanged - rustc 1.30.0-nightly (7061b2775 2018-08-28)

% cargo test set_remove
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running target/debug/deps/collection-c314c206089f5030

running 1 test
test ops::set::tests::set_remove ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out

   Doc-tests collection

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

% rustup override set nightly-2018-08-30-x86_64-unknown-linux-gnu
info: using existing install for 'nightly-2018-08-30-x86_64-unknown-linux-gnu'
info: override toolchain for '/home/pnkfelix/Dev/Mozilla/issue54477/collection' set to 'nightly-2018-08-30-x86_64-unknown-linux-gnu'

  nightly-2018-08-30-x86_64-unknown-linux-gnu unchanged - rustc 1.30.0-nightly (02cb8f2a4 2018-08-29)

% cargo test set_remove
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running target/debug/deps/collection-648e1050d7e72c52

running 1 test
test ops::set::tests::set_remove ... FAILED

failures:

---- ops::set::tests::set_remove stdout ----
thread 'ops::set::tests::set_remove' panicked at 'assertion failed: `(left == right)`
  left: `None`,
 right: `Some(69739)`', src/ops/set.rs:191:13
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    ops::set::tests::set_remove

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 26 filtered out

error: test failed, to rerun pass '--lib'
%
