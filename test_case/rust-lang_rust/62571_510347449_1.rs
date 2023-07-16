none
Documenting foo v0.1.0 (/tmp/tmp.AjQQzlRVZ2/foo)
thread 'rustc' panicked at 'unexpected item body MaybeEmphasis(2, true, false)', /cargo/registry/src/github.com-1ecc6299db9ec823/pulldown-cmark-0.5.2/src/parse.rs:2418:14
stack backtrace:
   0:     0x7f8f9326519b - backtrace::backtrace::libunwind::trace::hd732a1add0f3c38f
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1:     0x7f8f9326519b - backtrace::backtrace::trace_unsynchronized::h4791df561e50498d
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2:     0x7f8f9326519b - std::sys_common::backtrace::_print::h5fa2d794cb884684
                               at src/libstd/sys_common/backtrace.rs:47
   3:     0x7f8f9326519b - std::sys_common::backtrace::print::h7cd3adf88f405cef
                               at src/libstd/sys_common/backtrace.rs:36
   4:     0x7f8f9326519b - std::panicking::default_hook::{{closure}}::h2b88c1fac98b01ce
                               at src/libstd/panicking.rs:200
   5:     0x7f8f93264e77 - std::panicking::default_hook::h983f8b3138244f75
                               at src/libstd/panicking.rs:214
   6:     0x7f8f93265910 - std::panicking::rust_panic_with_hook::h94f6d90998bef5c8
                               at src/libstd/panicking.rs:477
   7:     0x7f8f93265492 - std::panicking::continue_panic_fmt::hf49c6a853e8dcac9
                               at src/libstd/panicking.rs:384
   8:     0x7f8f932653df - std::panicking::begin_panic_fmt::ha8d124bef3f21240
                               at src/libstd/panicking.rs:339
   9:     0x560e43064143 - pulldown_cmark::parse::item_to_event::h95012e9d04fec4c5
  10:     0x560e4306423d - <pulldown_cmark::parse::Parser as core::iter::traits::iterator::Iterator>::next::hded5130fc70fc34f
  11:     0x560e42e87f1d - rustdoc::passes::look_for_tests::ha49ad2c19398a144
  12:     0x560e42ee68e9 - <rustdoc::passes::private_items_doc_tests::PrivateItemDocTestLinter as rustdoc::fold::DocFolder>::fold_item::h2c34c33c7f518ff8
  13:     0x560e42de4bb5 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter::h1f5a786c5fe07bf7
  14:     0x560e42ee386f - rustdoc::fold::DocFolder::fold_inner_recur::h9d0af8fea7310887
  15:     0x560e42ee6b41 - <rustdoc::passes::private_items_doc_tests::PrivateItemDocTestLinter as rustdoc::fold::DocFolder>::fold_item::h2c34c33c7f518ff8
  16:     0x560e42ee63f4 - rustdoc::passes::private_items_doc_tests::check_private_items_doc_tests::h1a35ec8426427953
  17:     0x560e42e588e2 - rustc::ty::context::tls::enter_global::h004923b303546763
  18:     0x560e42ebe915 - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h0d82a7876664b846
  19:     0x7f8f967ef0da - rustc_interface::passes::create_global_ctxt::{{closure}}::h14ca905422d068e3
  20:     0x560e42ec189a - rustc_interface::interface::run_compiler_in_existing_thread_pool::hbef0f1e9fa8ef3ff
  21:     0x560e42fbb0e7 - rustdoc::core::run_core::h557abb113278db14
  22:     0x560e42ee78f9 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h1c3737142ce4c9e6
  23:     0x560e42e49c14 - std::panicking::try::do_call::h3feb55ab80dbb052
  24:     0x7f8f9327645a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:82
  25:     0x560e42eea0c3 - rustc_driver::report_ices_to_stderr_if_any::hb507e043ac4ab7f6
  26:     0x560e42dbc724 - rustdoc::main_options::h73fd32b381361a98
  27:     0x560e42e90045 - std::thread::local::LocalKey<T>::with::h7b42eea3105d8426
  28:     0x560e42ee9991 - scoped_tls::ScopedKey<T>::set::h8ad6cbf25bee106b
  29:     0x560e42ea4692 - syntax::with_globals::ha68beff4b747d351
  30:     0x560e42fa31a3 - std::sys_common::backtrace::__rust_begin_short_backtrace::hc52e7f8752451ef6
  31:     0x7f8f9327645a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:82
  32:     0x560e42d9afe9 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h8af64b4dfe60ea20
  33:     0x7f8f93248e1f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hfb7da7c10e37eb96
                               at /rustc/0beb2ba16a08dfa01569b5f4644da315dc4c806c/src/liballoc/boxed.rs:746
  34:     0x7f8f93275140 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hfa4f87915138ff10
                               at /rustc/0beb2ba16a08dfa01569b5f4644da315dc4c806c/src/liballoc/boxed.rs:746
  35:     0x7f8f93275140 - std::sys_common::thread::start_thread::h15a2ed152517f722
                               at src/libstd/sys_common/thread.rs:13
  36:     0x7f8f93275140 - std::sys::unix::thread::Thread::new::thread_start::h006cc43581e90d8f
                               at src/libstd/sys/unix/thread.rs:79
  37:     0x7f8f931ab182 - start_thread
  38:     0x7f8f930bab1f - __clone
  39:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0-nightly (0beb2ba16 2019-07-02) running on x86_64-unknown-linux-gnu
