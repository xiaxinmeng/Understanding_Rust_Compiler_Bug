text
(gdb) bt
#0  0x000002aa00034064 in std::sys::unix::backtrace::tracing::imp::trace_fn ()
#1  0x000003fffdd0a0ac in _Unwind_Backtrace () from /lib64/libgcc_s.so.1
#2  0x000002aa00033fbc in std::sys::unix::backtrace::tracing::imp::unwind_backtrace ()
#3  0x000002aa00039872 in std::sys_common::backtrace::print ()
#4  0x000002aa00029252 in std::panicking::default_hook::{{closure}} ()
#5  0x000002aa00028ede in std::panicking::default_hook ()
#6  0x000002aa0002979c in std::panicking::rust_panic_with_hook ()
#7  0x000002aa000180c6 in std::panicking::begin_panic (msg=..., file_line_col=0x2aa00063d30) at /home/nfs/jistone/rust/src/libstd/panicking.rs:391
#8  0x000002aa0001d4e4 in backtrace::foo () at ./src/test/run-pass/backtrace.rs:25
#9  0x000002aa0001ec88 in backtrace::main () at ./src/test/run-pass/backtrace.rs:111
#10 0x000002aa00017f3c in std::rt::lang_start::{{closure}} () at /home/nfs/jistone/rust/src/libstd/rt.rs:74
#11 0x000002aa0002933a in std::panicking::try::do_call ()
#12 0x000002aa00042ddc in __rust_try ()
Backtrace stopped: previous frame identical to this frame (corrupt stack?)
