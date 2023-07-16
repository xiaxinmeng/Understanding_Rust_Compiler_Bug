
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', ../src/libcore/option.rs:326
stack backtrace:
   1:     0x7f2c90370e09 - std::sys::backtrace::tracing::imp::write::h482d45d91246faa2
   2:     0x7f2c903814fc - std::panicking::default_hook::_{{closure}}::h89158f66286b674e
   3:     0x7f2c9037f807 - std::panicking::default_hook::h9e30d428ee3b0c43
   4:     0x7f2c90380018 - std::panicking::rust_panic_with_hook::h2224f33fb7bf2f4c
   5:     0x7f2c9037feb2 - std::panicking::begin_panic::hcb11a4dc6d779ae5
   6:     0x7f2c9037fde0 - std::panicking::begin_panic_fmt::h310416c62f3935b3
   7:     0x7f2c9037fd61 - rust_begin_unwind
   8:     0x7f2c903eb42f - core::panicking::panic_fmt::hc5789f4e80194729
   9:     0x7f2c903eb35b - core::panicking::panic::h1953378f4b37b561
  10:     0x7f2c8ea7f0dc - rustc_typeck::check::compare_method::compare_impl_method::_{{closure}}::h08bef90d287aed42
  11:     0x7f2c8e9db0f9 - rustc_typeck::check::compare_method::compare_impl_method::he3538453c2efe177
  12:     0x7f2c8e9f0082 - rustc_typeck::check::check_impl_items_against_trait::hb105ca8bb42a0b3d
  13:     0x7f2c8e9ed602 - rustc_typeck::check::check_item_type::hba3733559d49be1e
  14:     0x7f2c8e9e5e4b - _<rustc_typeck..check..CheckItemTypesVisitor<'a, 'tcx> as rustc..hir..intravisit..Visitor<'tcx>>::visit_item::h369cacffbaa18923
  15:     0x7f2c8e9e7d4b - rustc_typeck::check::check_item_types::h81e44a5d40f450fd
  16:     0x7f2c8ea5f4c3 - rustc_typeck::check_crate::h8ec8b2f490f5dc28
  17:     0x7f2c90760d13 - rustc_driver::driver::phase_3_run_analysis_passes::_{{closure}}::h45e03cee16dcf298
  18:     0x7f2c9072d756 - rustc_driver::driver::phase_3_run_analysis_passes::hc0de40cea97d81a1
  19:     0x7f2c90719f42 - rustc_driver::driver::compile_input::hd9ecc57abd3cba85
  20:     0x7f2c9074c966 - rustc_driver::run_compiler::h184264500271cc39
  21:     0x7f2c906928c2 - std::panicking::try::do_call::h17a7a17ad7240c5c
  22:     0x7f2c9038f866 - __rust_maybe_catch_panic
  23:     0x7f2c906ac2aa - _<F as alloc..boxed..FnBox<A>>::call_box::h93f9128277b2964a
  24:     0x7f2c9037dbd2 - std::sys::thread::Thread::new::thread_start::he0bf102845911132
  25:     0x7f2c886b10a3 - start_thread
  26:     0x7f2c8ffc087c - clone
  27:                0x0 - <unknown>
