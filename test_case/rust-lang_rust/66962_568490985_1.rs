
error: internal compiler error: constant in type had an ignored error: TooGeneric
  --> src/test_const.rs:27:5
   |
27 |     bitfields: [usize; SIZE / 64]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:347:17
stack backtrace:
   0:     0x7f6ac90892e4 - backtrace::backtrace::libunwind::trace::he29c1eef22099f24
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1:     0x7f6ac90892e4 - backtrace::backtrace::trace_unsynchronized::ha36b80e516498eae
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2:     0x7f6ac90892e4 - std::sys_common::backtrace::_print_fmt::hb70f67b9deb9591b
                               at src/libstd/sys_common/backtrace.rs:84
   3:     0x7f6ac90892e4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf6ea30c21a2e486e
                               at src/libstd/sys_common/backtrace.rs:61
   4:     0x7f6ac90c176c - core::fmt::write::hfd06ed55ab82c6c1
                               at src/libcore/fmt/mod.rs:1057
   5:     0x7f6ac907d717 - std::io::Write::write_fmt::h8c600764d032d6f3
                               at src/libstd/io/mod.rs:1426
   6:     0x7f6ac908d78e - std::sys_common::backtrace::_print::h2ccb403bde2af48d
                               at src/libstd/sys_common/backtrace.rs:65
   7:     0x7f6ac908d78e - std::sys_common::backtrace::print::hde1d2976285bb6d7
                               at src/libstd/sys_common/backtrace.rs:50
   8:     0x7f6ac908d78e - std::panicking::default_hook::{{closure}}::h5f8dbaddde324745
                               at src/libstd/panicking.rs:193
   9:     0x7f6ac908d481 - std::panicking::default_hook::h353c9f7e159c6169
                               at src/libstd/panicking.rs:210
  10:     0x7f6ac9610b23 - rustc_driver::report_ice::h1312d9253207031a
  11:     0x7f6abb348b88 - <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call::hd34b6e757f60fcb9
                               at /rustc/9b98af84c4aa66392236fff59c86da2130d46d46/src/liballoc/boxed.rs:1036
  12:     0x7f6abb34f204 - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::h78bcbeb91c63709f
                               at /rustc/9b98af84c4aa66392236fff59c86da2130d46d46/src/libproc_macro/bridge/client.rs:305
  13:     0x7f6ac908df40 - std::panicking::rust_panic_with_hook::h2106ae29891f5da1
                               at src/libstd/panicking.rs:475
  14:     0x7f6acb7a5a05 - std::panicking::begin_panic::h93fe85ac6a0e3437
  15:     0x7f6acb7d691c - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h55cf2ea02b0742fd
  16:     0x7f6ac95dd986 - core::ptr::real_drop_in_place::h392a37dbd6e46951
  17:     0x7f6ac95f46f3 - core::ptr::real_drop_in_place::hea055a76d2f2dbd0
  18:     0x7f6ac95f417c - core::ptr::real_drop_in_place::he6fe08f725a1ff55
  19:     0x7f6ac95d7044 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h73894470eb0ec5df
  20:     0x7f6ac95b46b2 - std::thread::local::LocalKey<T>::with::h495fb06c01f6023a
  21:     0x7f6ac95a620e - scoped_tls::ScopedKey<T>::set::h472b490a2228a3c9
  22:     0x7f6ac962e264 - syntax::with_globals::hc9ab57e6f4791057
  23:     0x7f6ac95a69a0 - std::sys_common::backtrace::__rust_begin_short_backtrace::h65d85a8a2939bf16
  24:     0x7f6ac909ea6a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:78
  25:     0x7f6ac95beeb9 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h603ca4a944c4f841
  26:     0x7f6ac906ef6f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h480530d6a186f051
                               at /rustc/9b98af84c4aa66392236fff59c86da2130d46d46/src/liballoc/boxed.rs:1022
  27:     0x7f6ac909d490 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h45ae71d65ffe3628
                               at /rustc/9b98af84c4aa66392236fff59c86da2130d46d46/src/liballoc/boxed.rs:1022
  28:     0x7f6ac909d490 - std::sys_common::thread::start_thread::h9d0d044e93c04e51
                               at src/libstd/sys_common/thread.rs:13
  29:     0x7f6ac909d490 - std::sys::unix::thread::Thread::new::thread_start::h59144ad61d4caeef
                               at src/libstd/sys/unix/thread.rs:80
  30:     0x7f6ac8fe64c0 - start_thread
  31:     0x7f6ac8f04553 - __clone
  32:                0x0 - <unknown>
