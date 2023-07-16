
Thread 2 (Thread 0x76d512f0 (LWP 19622)):
#0  __GI___tls_get_addr (ti=0xdc38b4) at dl-tls.c:828
#1  0x00bd0a64 in core::option::Option<T>::as_ref () at /rustc/a7f28678bbf4e16893bb6a718e427504167a9494/src/libcore/option.rs:266
#2  std::thread::local::lazy::LazyKeyInner<T>::get () at src/libstd/thread/local.rs:279
#3  std::thread::local::fast::Key<T>::get () at src/libstd/thread/local.rs:409
#4  std::panicking::update_panic_count::PANIC_COUNT::__getit () at src/libstd/thread/local.rs:176
#5  std::thread::local::LocalKey<T>::try_with () at src/libstd/thread/local.rs:254
#6  std::thread::local::LocalKey<T>::with () at src/libstd/thread/local.rs:234
#7  std::panicking::update_panic_count () at src/libstd/panicking.rs:226
#8  std::panicking::panicking () at src/libstd/panicking.rs:303
#9  std::thread::panicking () at src/libstd/thread/mod.rs:724
#10 std::sys_common::poison::Flag::done () at src/libstd/sys_common/poison.rs:36
#11 <std::sync::mutex::MutexGuard<T> as core::ops::drop::Drop>::drop () at src/libstd/sync/mutex.rs:448
#12 core::ptr::real_drop_in_place () at /rustc/a7f28678bbf4e16893bb6a718e427504167a9494/src/libcore/ptr/mod.rs:175
#13 std::thread::park () at src/libstd/thread/mod.rs:917
#14 0x00bde4b4 in std::sync::mpsc::blocking::WaitToken::wait () at src/libstd/sync/mpsc/blocking.rs:71
#15 0x00baf508 in std::sync::mpsc::oneshot::Packet<T>::recv ()
#16 0x00baddb8 in std::sync::mpsc::Receiver<T>::recv ()
#17 0x00bb2d38 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
#18 0x00bb1b44 in std::panicking::try::do_call ()
#19 0x00bebf88 in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:82
#20 0x00bb2110 in core::ops::function::FnOnce::call_once{{vtable-shim}} ()
#21 0x00bcfb74 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/a7f28678bbf4e16893bb6a718e427504167a9494/src/liballoc/boxed.rs:766
#22 0x00beb48c in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/a7f28678bbf4e16893bb6a718e427504167a9494/src/liballoc/boxed.rs:766
#23 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#24 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:79
#25 0x76f56494 in start_thread (arg=0x76d512f0) at pthread_create.c:486
Backtrace stopped: Cannot access memory at address 0x21046

Thread 1 (Thread 0x76ff4010 (LWP 19619)):
#0  futex_reltimed_wait_cancelable (private=0, reltime=0x7effb8a0, expected=0, futex_word=0xdcd3b8) at ../sysdeps/unix/sysv/linux/futex-internal.h:142
#1  __pthread_cond_wait_common (abstime=0x7effb910, mutex=0x2, cond=0xdcd390) at pthread_cond_wait.c:533
#2  __pthread_cond_timedwait (cond=0xdcd390, mutex=0x2, abstime=0x7effb910) at pthread_cond_wait.c:667
#3  0x00be44bc in std::sys::unix::condvar::Condvar::wait_timeout () at src/libstd/sys/unix/condvar.rs:100
#4  0x00bd0df8 in std::sys_common::condvar::Condvar::wait_timeout () at src/libstd/sys_common/condvar.rs:51
#5  std::sync::condvar::Condvar::wait_timeout () at src/libstd/sync/condvar.rs:405
#6  std::thread::park_timeout () at src/libstd/thread/mod.rs:1006
#7  0x00bde5f4 in std::sync::mpsc::blocking::WaitToken::wait_max_until () at src/libstd/sync/mpsc/blocking.rs:82
#8  0x00baf4d8 in std::sync::mpsc::oneshot::Packet<T>::recv ()
#9  0x00badb40 in std::sync::mpsc::Receiver<T>::recv_timeout ()
#10 0x00bad3d0 in jobserver::imp::Helper::join ()
#11 0x00bb0d70 in <jobserver::HelperThread as core::ops::drop::Drop>::drop ()
#12 0x006f5218 in core::ptr::real_drop_in_place ()
#13 0x00719b20 in cargo::core::compiler::job_queue::JobQueue::execute ()
#14 0x007d9050 in cargo::core::compiler::context::Context::compile ()
#15 0x00550a20 in cargo::ops::cargo_compile::compile_ws ()
#16 0x0054f878 in cargo::ops::cargo_compile::compile ()
#17 0x00490a44 in cargo::commands::check::exec ()
#18 0x0045a184 in cargo::cli::main ()
#19 0x0049b270 in cargo::main ()
#20 0x004787cc in std::rt::lang_start::{{closure}} ()
#21 0x00be2350 in std::rt::lang_start_internal::{{closure}} () at src/libstd/rt.rs:49
#22 std::panicking::try::do_call () at src/libstd/panicking.rs:296
#23 0x00bebf88 in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:82
#24 0x00be2e7c in std::panicking::try () at src/libstd/panicking.rs:275
#25 std::panic::catch_unwind () at src/libstd/panic.rs:394
#26 std::rt::lang_start_internal () at src/libstd/rt.rs:48
#27 0x0049cff8 in main ()
