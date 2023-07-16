
#0  __futex_abstimed_wait_common64 (private=0x565f7170, cancel=0x1, abstime=0x7fffffff82e8, op=0x89, expected=0x0, futex_word=0x5555565fddfc) at futex-internal.c:57
57      in futex-internal.c
gef➤  bt
#0  __futex_abstimed_wait_common64 (private=0x565f7170, cancel=0x1, abstime=0x7fffffff82e8, op=0x89, expected=0x0, futex_word=0x5555565fddfc) at futex-internal.c:57
#1  __futex_abstimed_wait_common (cancel=0x1, private=0x565f7170, abstime=0x7fffffff82e8, clockid=0x7fff, expected=0x0, futex_word=0x5555565fddfc) at futex-internal.c:87
#2  __GI___futex_abstimed_wait_cancelable64 (futex_word=futex_word@entry=0x5555565fddfc, expected=expected@entry=0x0, clockid=clockid@entry=0x1, abstime=abstime@entry=0x7fffffff82e8, private=private@entry=0x0) at futex-internal.c:139
#3  0x00007ffff7d0ed7e in __pthread_cond_wait_common (abstime=0x7fffffff82e8, clockid=0x1, mutex=0x5555566dab50, cond=0x5555565fddd0) at pthread_cond_wait.c:504
#4  ___pthread_cond_timedwait64 (cond=0x5555565fddd0, mutex=0x5555566dab50, abstime=0x7fffffff82e8) at pthread_cond_wait.c:653
#5  0x0000555555f04105 in std::sys::unix::condvar::Condvar::wait_timeout () at library/std/src/sys/unix/condvar.rs:114
#6  0x0000555555ecd9a2 in jobserver::imp::Helper::join ()
#7  0x0000555555ece9c3 in <jobserver::HelperThread as core::ops::drop::Drop>::drop ()
#8  0x00005555559cee07 in <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
#9  0x000055555575619c in crossbeam_utils::thread::scope ()
#10 0x000055555568479c in cargo::core::compiler::job_queue::JobQueue::execute ()
#11 0x000055555562a0c4 in cargo::core::compiler::context::Context::compile ()
#12 0x000055555595361d in cargo::ops::cargo_compile::compile_ws ()
#13 0x0000555555953356 in cargo::ops::cargo_compile::compile ()
#14 0x00005555559a6c3e in cargo::ops::cargo_test::run_tests ()
#15 0x00005555555d3c13 in cargo::commands::test::exec ()
#16 0x00005555555c5cbd in cargo::cli::main ()
#17 0x00005555555ee715 in cargo::main ()
#18 0x00005555555b6813 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
#19 0x00005555555e6eb9 in std::rt::lang_start::{{closure}} ()
#20 0x0000555555f0283a in core::ops::function::impls::{impl#2}::call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> () at /rustc/9bb77da74dac4768489127d21e32db19b59ada5b/library/core/src/ops/function.rs:259
#21 std::panicking::try::do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> () at library/std/src/panicking.rs:403
#22 std::panicking::try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> () at library/std/src/panicking.rs:367
#23 std::panic::catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> () at library/std/src/panic.rs:129
#24 std::rt::lang_start_internal::{closure#2} () at library/std/src/rt.rs:45
#25 std::panicking::try::do_call<std::rt::lang_start_internal::{closure#2}, isize> () at library/std/src/panicking.rs:403
#26 std::panicking::try<isize, std::rt::lang_start_internal::{closure#2}> () at library/std/src/panicking.rs:367
#27 std::panic::catch_unwind<std::rt::lang_start_internal::{closure#2}, isize> () at library/std/src/panic.rs:129
#28 std::rt::lang_start_internal () at library/std/src/rt.rs:45
#29 0x00005555555f0902 in main ()
