
#0  rust_task_fail (task=0x6106f0, expr=0x26dd5c20 "explicit failure", file=0x20f0ba10 "/home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/libcore/run.rs", line=343)
    at /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/rt/rust_task.cpp:85
#1  0x00007ffff47b89f1 in __morestack () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustrt.so
#2  0x00007ffff47aacb6 in call_on_c_stack (fn_ptr=0x7ffff47a9dc0, args=0x6333f0, this=0x6106f0) at /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/rt/rust_task.h:470
#3  call_upcall_on_c_stack (fn_ptr=0x7ffff47a9dc0, args=0x6333f0, task=<optimized out>) at /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/rt/rust_upcall.cpp:45
#4  upcall_fail (expr=<optimized out>, file=0x20f0ba10 "/home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/libcore/run.rs", line=343)
    at /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/rt/rust_upcall.cpp:128
#5  0x00007ffff79d19ab in sys::begin_unwind_::_f79ba45ac636dce::_06 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
#6  0x00007ffff79d1952 in sys::begin_unwind::anon::anon::expr_fn_14663 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
#7  0x00007ffff7919441 in sys::begin_unwind::_701b8ec61f97ead::_06 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
#8  0x00007ffff79cbaa2 in run::program_output::_3596fc6920d4f35::_06 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
#9  0x00007ffff7a1a214 in __morestack () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
#10 0x00007ffff68e0bad in back::link::link_binary::_483345abdae71288::_06 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#11 0x00007ffff66a9835 in util::common::time_55885::_71a86f54b8835f8b::_06 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#12 0x00007ffff69472b2 in driver::driver::compile_rest::_87c0f340aa21a39::_06 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#13 0x00007ffff6987e84 in __morestack () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#14 0x00007ffff6949738 in driver::driver::compile_upto::_84743710faafb0af::_06 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#15 0x00007ffff6987e84 in __morestack () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#16 0x00007ffff6949b7a in driver::driver::compile_input::_a6b0fe22676e6b::_06 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#17 0x00007ffff697931a in run_compiler::_9de519bbeb75837::_06 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#18 0x00007ffff6985951 in monitor::anon::expr_fn_81957 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#19 0x00007ffff698217c in task::__extensions__::try_81478::anon::expr_fn_81741 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#20 0x00007ffff6987e84 in __morestack () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#21 0x00007ffff799a38e in task::spawn::spawn_raw::make_child_wrapper::anon::expr_fn_12071 () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
#22 0x00007ffff7a1a214 in __morestack () from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
#23 0x00007ffff47a9534 in task_start_wrapper (a=<optimized out>) at /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/rt/rust_task.cpp:162
#24 0x0000000000000000 in ?? ()
