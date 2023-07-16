
Executing with sh: make  check TESTNAME=test_process_mask
cfg: version 1.3.0-dev (1b5df5a79 2015-07-29)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
cfg: no xelatex found, disabling LaTeX docs
cfg: no pandoc found, omitting PDF and EPUB docs
cfg: including test rules
cfg: javac not available, skipping lexer test...
run: x86_64-unknown-linux-gnu/stage2/test/stdtest-x86_64-unknown-linux-gnu

running 1 test
thread '<unnamed>' panicked at 'assertion failed: ret == 0', src/libstd/sys/unix/process.rs:499
stack backtrace:
   1:     0x7fb91d34e8b3 - sys::backtrace::write::h118c75566581584bLns
   2:     0x7fb91d35735b - panicking::on_panic::hb29e0c92e68b6099Nxx
   3:     0x7fb91d18c67e - rt::unwind::begin_unwind_inner::h16084ed3d9b2a0caBex
   4:     0x7fb91d18c561 - rt::unwind::begin_unwind::h4876894998304443716
   5:     0x7fb91d35203e - sys::process::tests::test_process_mask::h7f10df19f558087a1wv
   6:     0x7fb91d37bc1b - boxed::F.FnBox<A>::call_box::h3137439688241643459
   7:     0x7fb91d37e4a2 - boxed::F.FnBox<A>::call_box::h894454766707528792
   8:     0x7fb91d37c3a9 - rt::unwind::try
::try_fn::h12391789102408719320
   9:     0x7fb91d3aa15f - __rust_try_inner
  10:     0x7fb91d3aa19a - __rust_try
  11:     0x7fb91d3a5497 - rt::unwind::try::inner_try::h4b94860011c08d91AYw
  12:     0x7fb91d37c5c8 - boxed::F.FnBox<A>::call_box::h4082951101316925067
  13:     0x7fb91d3a9311 - sys::thread::Thread::new::thread_start::h866d84d0c1e32215g8v
  14:     0x7fb91cb200a3 - start_thread
  15:     0x7fb91c43704c - clone
  16:                0x0 - <unknown>
thread '<main>' panicked at 'Some tests failed', src/libtest/lib.rs:253
stack backtrace:
test sys::process::tests::test_process_mask ... FAILED

failures:

failures:
    sys::process::tests::test_process_mask

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

   1:     0x7fb91d3a6d8e - sys::backtrace::write::h3ed48d9b04182c33Cys
   2:     0x7fb91d3aa862 - panicking::on_panic::h252e2f106e1626c84mx
   3:     0x7fb91d39d5ae - rt::unwind::begin_unwind_inner::h90851cc7e0eaffa9H2w
   4:     0x7fb91d35bb61 - rt::unwind::begin_unwind::h9263853665045484523
   5:     0x7fb91d35d51c - test_main::h499a4ab67aa279d2I1a
   6:     0x7fb91d363a93 - test_main_static::h3fd6cc5020f829d7d4a

   7:     0x7fb91d35a8b9 - __test::main::hcea93956d2b72f1aD7x
   8:     0x7fb91d3aa15f - __rust_try_inner
   9:     0x7fb91d3aa19a - __rust_try
  10:     0x7fb91d3ac517 - rt::lang_start::ha3f98d17b257b53b0hx
  11:     0x7fb91c372b44 - __libc_start_main
  12:     0x7fb91d1880f8 - <unknown>
  13:                0x0 - <unknown>
make: *** [tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-std.ok] Error 101
rust/mk/tests.mk:449: recipe for target 'tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-std.ok' failed
