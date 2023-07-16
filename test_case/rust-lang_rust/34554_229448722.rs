
thread '<unnamed>' panicked at 'called `Option::unwrap()` on a `None` value', ../src/libcore/option.rs:325
stack backtrace:
   1:     0x7f0e26e7adef - std::sys::backtrace::tracing::imp::write::h6528da8103c51ab9
   2:     0x7f0e26e88ddb - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hbe741a5cc3c49508
   3:     0x7f0e26e88a0f - std::panicking::default_hook::he0146e6a74621cb4
   4:     0x7f0e26e4e67e - std::panicking::rust_panic_with_hook::h983af77c1a2e581b
   5:     0x7f0e26e89021 - std::panicking::begin_panic::he426e15a3766089a
   6:     0x7f0e26e506da - std::panicking::begin_panic_fmt::hdddb415186c241e7
   7:     0x7f0e26e88fbe - rust_begin_unwind
   8:     0x7f0e26ed1fef - core::panicking::panic_fmt::hf4e16cb7f0d41a25
   9:     0x7f0e26ed2ae8 - core::panicking::panic::h907815f47e914305
  10:     0x7f0e274826f3 - rustdoc::html::render::render_impl::doctraititem::h32eec0f432e5269b
  11:     0x7f0e27481186 - rustdoc::html::render::render_impl::h8405bc03794cc756
  12:     0x7f0e2747eb74 - rustdoc::html::render::render_assoc_items::h4704ada9fa7cc3e3
  13:     0x7f0e274774fa - _<rustdoc..html..render..Item<'a> as core..fmt..Display>::fmt::h9c0a28f83547182c
  14:     0x7f0e26efc73a - core::fmt::write::hdac96890aec66a9a
  15:     0x7f0e2742e722 - std::io::Write::write_fmt::h8f7bba2797a77d53
  16:     0x7f0e2746fe8f - rustdoc::html::render::Context::item::render::h4dbcafd1a53d9010
  17:     0x7f0e27394ade - rustdoc::html::render::run::h804448c4d17b6d11
  18:     0x7f0e27375185 - rustdoc::main_args::h92ed2068a7dd9783
  19:     0x7f0e27370b6a - std::panicking::try::call::ha3f28c009d90b0a9
  20:     0x7f0e26e9729b - __rust_try
  21:     0x7f0e26e9723e - __rust_maybe_catch_panic
  22:     0x7f0e27371022 - _<F as alloc..boxed..FnBox<A>>::call_box::hb90cf37bd6edcde3
  23:     0x7f0e26e86f04 - std::sys::thread::Thread::new::thread_start::h9c883b6d445ece46
  24:     0x7f0e1f0d5aa0 - start_thread
  25:     0x7f0e26ae393c - clone
  26:                0x0 - <unknown>
error: Could not document `hyper`.
