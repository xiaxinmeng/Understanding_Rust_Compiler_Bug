rust
                               at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x56287ebb9a54 - std::sys_common::backtrace::_print::hd8a1b72dcf3955ef
                               at /checkout/src/libstd/sys_common/backtrace.rs:71
   2:     0x56287ebc149c - std::panicking::default_hook::{{closure}}::h5ff605bba7612658
                               at /checkout/src/libstd/sys_common/backtrace.rs:60
                               at /checkout/src/libstd/panicking.rs:355
   3:     0x56287ebc1064 - std::panicking::default_hook::h9bc4f6dfee57d6bd
                               at /checkout/src/libstd/panicking.rs:371
   4:     0x56287ebc18eb - std::panicking::rust_panic_with_hook::hdc01585dc2bf7122
                               at /checkout/src/libstd/panicking.rs:549
   5:     0x56287ebc17c4 - std::panicking::begin_panic::hf84f4975d9f9b642
                               at /checkout/src/libstd/panicking.rs:511
   6:     0x56287ebc16f9 - std::panicking::begin_panic_fmt::hcc3f360b2ba80419
                               at /checkout/src/libstd/panicking.rs:495
   7:     0x56287e3cbbc6 - <std::sync::mpsc::shared::Packet<T>>::decrement::he4fa9520181c5c85
                               at /checkout/src/libstd/macros.rs:51
   8:     0x56287e3c806f - <std::sync::mpsc::shared::Packet<T>>::recv::h3c95f5bc336537aa
                               at /checkout/src/libstd/sync/mpsc/shared.rs:232
   9:     0x56287e3ad379 - <std::sync::mpsc::Receiver<T>>::recv_max_until::h950909094e0767d9
                               at /checkout/src/libstd/sync/mpsc/mod.rs:966
  10:     0x56287e3acc85 - <std::sync::mpsc::Receiver<T>>::recv_timeout::hf72a64a0530efaa1
                               at /checkout/src/libstd/sync/mpsc/mod.rs:940
  11:     0x56287e466841 - <mould_extension::ConnectExtensionWorker as mould::worker::Worker<T>>::realize::hf8b7190433e70336
                               at /home/denis/vxrevenue/cloud/sub/mould-extension/src/lib.rs:129
  12:     0x56287e41cd77 - mould::server::process_session::{{closure}}::h0572e63ea7bd3be9
                               at /home/denis/vxrevenue/cloud/sub/mould/src/server.rs:83
  13:     0x56287e41b69a - mould::server::process_session::h54610d99cf99088f
                               at /home/denis/vxrevenue/cloud/sub/mould/src/server.rs:44
  14:     0x56287e41f76b - mould::server::wsmould::start::{{closure}}::hb36d2fb80ee5ded6
                               at /home/denis/vxrevenue/cloud/sub/mould/src/server.rs:272
  15:     0x56287e469345 - <std::panic::AssertUnwindSafe<F> as core::ops::FnOnce<()>>::call_once::h8ef904bc75108aeb
                               at /checkout/src/libstd/panic.rs:296
  16:     0x56287e3a2e7a - std::panicking::try::do_call::h0979d3031b45f486
                               at /checkout/src/libstd/panicking.rs:454
  17:     0x56287ebc897a - __rust_maybe_catch_panic
                               at /checkout/src/libpanic_unwind/lib.rs:98
  18:     0x56287e3a26de - std::panicking::try::h42b9334978084c46
                               at /checkout/src/libstd/panicking.rs:433
  19:     0x56287e39d4a3 - std::panic::catch_unwind::h5ea213ef0eb7edd1
                               at /checkout/src/libstd/panic.rs:361
  20:     0x56287e3a1a86 - std::thread::Builder::spawn::{{closure}}::h1288ffa1c4d83635
                               at /checkout/src/libstd/thread/mod.rs:360
  21:     0x56287e3fca66 - <F as alloc::boxed::FnBox<A>>::call_box::h1b125a486a246990
                               at /checkout/src/liballoc/boxed.rs:640
  22:     0x56287ebc0714 - std::sys::imp::thread::Thread::new::thread_start::h75b208405df6dcf1
                               at /checkout/src/liballoc/boxed.rs:650
                               at /checkout/src/libstd/sys_common/thread.rs:21
                               at /checkout/src/libstd/sys/unix/thread.rs:84
  23:     0x7f7d554096c9 - start_thread
  24:     0x7f7d54f2cf7e - clone
  25:                0x0 - <unknown>
