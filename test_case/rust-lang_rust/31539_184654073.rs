
error: test run failed!
status: exit code: 101
command: x86_64-unknown-linux-gnu/test/run-pass/backtrace.stage2-x86_64-unknown-linux-gnu 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'bad output: thread '<main>' panicked at 'explicit panic', /home/travis/build/rust-lang/rust/src/test/run-pass/backtrace.rs:23
stack backtrace:
   1:     0x2b4313b5fec0 - std::sys::backtrace::tracing::imp::write::h5cb8831505517f1a
   2:     0x2b4313b68e3b - std::panicking::default_handler::_$u7b$$u7b$closure$u7d$$u7d$::h7d4838ac3001375c
   3:     0x2b4313b68a08 - std::panicking::default_handler::heb8f66506b25fc53
   4:     0x2b4313b31ac6 - std::sys_common::unwind::begin_unwind_inner::h896acc46de4d4a1c
   5:     0x2b4313671e44 - std::sys_common::unwind::begin_unwind::h72328ba1025302f0
                        at src/libstd/sys/common/unwind/mod.rs:236
   6:     0x2b4313671dd3 - backtrace::foo::h2bfe25e162df4c7c
                        at /home/travis/build/rust-lang/rust/<std macros>:3
   7:     0x2b43136732a5 - backtrace::main::h1e5dc0fe5e6f026b
                        at src/test/run-pass/backtrace.rs:98
   8:     0x2b4313b68474 - std::sys_common::unwind::try::try_fn::hbb3b06ed012dffa6
   9:     0x2b4313b5d82b - __rust_try
  10:     0x2b4313b680b1 - std::rt::lang_start::ha4a60ae3cf761e9f
  11:     0x2b431424eec4 - __libc_start_main
  12:     0x2b4313671c28 - <unknown>
  13:                0x0 - <unknown>
', /home/travis/build/rust-lang/rust/src/test/run-pass/backtrace.rs:54
note: Run with `RUST_BACKTRACE=1` for a backtrace.

------------------------------------------
