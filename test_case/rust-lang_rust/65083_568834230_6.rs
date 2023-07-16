rust
thread 'rustc' panicked at 'LOOK at the backtrace', src/librustc/hir/lowering/item.rs:1340:13
stack backtrace:
   0:     0x7ff689a8d2d3 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd34406146e41daae
   1:     0x7ff689ac678d - core::fmt::write::h1e6401f1bc726ba8
   2:     0x7ff689a89d85 - std::io::Write::write_fmt::h2b25e3789d63012f
   3:     0x7ff689a63201 - std::panicking::default_hook::{{closure}}::hda34beae3510ea84
   4:     0x7ff689a62f13 - std::panicking::default_hook::h773eb09b9dfdee98
   5:     0x7ff689fe6fd3 - rustc_driver::report_ice::haae7e03a292ed2f5
   6:     0x7ff689a63a73 - std::panicking::rust_panic_with_hook::h9d6bc828824a2cf0
   7:     0x7ff68bb8b0d8 - std::panicking::begin_panic::h5ada958e28d63107
   8:     0x7ff68b71be78 - rustc::hir::lowering::item::<impl rustc::hir::lowering::LoweringContext>::lower_generics::ha5c29314c34b310f
   9:     0x7ff68b7138de - rustc::hir::lowering::item::<impl rustc::hir::lowering::LoweringContext>::lower_item::hc0804f4d7d372f39
  10:     0x7ff68b71f7e2 - rustc::hir::lowering::LoweringContext::with_hir_id_owner::h83083dbf072d6c73
  11:     0x7ff68bbb0487 - <rustc::hir::lowering::item::ItemLowerer as syntax::visit::Visitor>::visit_mod::h5f7158a54b74d646
  12:     0x7ff68bba49b5 - syntax::visit::walk_crate::h5826855d4e945a0f
  13:     0x7ff68b71c978 - rustc::hir::lowering::lower_crate::hf0597a212c2c188b
  14:     0x7ff68a14da20 - rustc_interface::passes::BoxedResolver::access::{{closure}}::h4a9e33b89496e5f7
  15:     0x7ff68a051c96 - rustc_interface::passes::configure_and_expand::{{closure}}::habdc65ecf0b0a45f
  16:     0x7ff68a14d1ff - rustc_interface::passes::BoxedResolver::access::h4e93696ea384734b
  17:     0x7ff68a0c39fb - rustc_interface::queries::Queries::lower_to_hir::h770ff6f3907c412b
  18:     0x7ff68a0c44bf - rustc_interface::queries::Queries::global_ctxt::h8d259a1a4277c38a
  19:     0x7ff689fad3c7 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h572f64630073ca22
  20:     0x7ff689f7a312 - std::thread::local::LocalKey<T>::with::h738412bb23904225
  21:     0x7ff689f739b1 - scoped_tls::ScopedKey<T>::set::h64e82e19b42e31f1
  22:     0x7ff689ff2be6 - syntax::with_globals::h5c0438bdced03c80
  23:     0x7ff689f74500 - std::sys_common::backtrace::__rust_begin_short_backtrace::hd8557f4984f7bc60
  24:     0x7ff689a97c6a - __rust_maybe_catch_panic
  25:     0x7ff689f8b9f9 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6d0984135f4e8a6a
  26:     0x7ff689a7259f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hdf5781f7ddd72291
  27:     0x7ff689a89fa0 - std::sys_common::thread::start_thread::h3b7aa4fd3951ea4c
  28:     0x7ff689a6eb56 - std::sys::unix::thread::Thread::new::thread_start::hdb632da9bc7d6c7d
  29:     0x7ff688f6c6db - start_thread
  30:     0x7ff68973c88f - __clone
  31:                0x0 - <unknown>

