
#0  0x00000055555e6914 in core::mem::transmute_copy<[lang::Foo; 1500], [core::mem::maybe_uninit::MaybeUninit<lang::Foo>; 1500]> () at /home/builder/.termux-build/rust/src/library/core/src/mem/mod.rs:1047
#1  0x00000055555e657c in core::array::iter::{impl#0}::into_iter<lang::Foo, 1500> ()
    at /home/builder/.termux-build/rust/src/library/core/src/array/iter.rs:73
#2  0x00000055555e6b90 in as_bin::tests::test_foo () at src/lib.rs:16
#3  0x00000055555e77d4 in as_bin::tests::test_foo::{closure#0} () at src/lib.rs:15
#4  0x00000055555e6d00 in core::ops::function::FnOnce::call_once<lang::tests::test_foo::{closure_env#0}, ()> ()
    at /home/builder/.termux-build/rust/src/library/core/src/ops/function.rs:250
#5  0x00000055556062b8 in test::__rust_begin_short_backtrace::h5295331c10838615 ()
#6  0x00000055556054b0 in test::run_test::run_test_inner::{{closure}}::hdb6e5fbd252316c8 ()
#7  0x00000055555e8028 in std::sys_common::backtrace::__rust_begin_short_backtrace::h62188ba109d57d3c ()
#8  0x0000005555609b24 in core::ops::function::FnOnce::call_once{{vtable.shim}}::hcb784459aeff44a4 ()
#9  0x00000055556282d4 in std::sys::unix::thread::Thread::new::thread_start::h573c53a4ca02e775 ()
#10 0x0000007ff2b4b72c in __pthread_start(void*) () from /apex/com.android.runtime/lib64/bionic/libc.so
#11 0x0000007ff2ae5020 in __start_thread () from /apex/com.android.runtime/lib64/bionic/libc.so
