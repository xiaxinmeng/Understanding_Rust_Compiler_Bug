
 Documenting glib v0.4.1 (file:///D:/eap/rust/0/glib)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', libcore\option.rs:335:21
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: mingw_set_invalid_parameter_handler
   1: mingw_set_invalid_parameter_handler
   2: mingw_set_invalid_parameter_handler
   3: mingw_set_invalid_parameter_handler
   4: mingw_set_invalid_parameter_handler
   5: mingw_set_invalid_parameter_handler
   6: mingw_set_invalid_parameter_handler
   7: mingw_set_invalid_parameter_handler
   8: mingw_set_invalid_parameter_handler
   9: mingw_set_invalid_parameter_handler
  10: mingw_set_invalid_parameter_handler
  11: mingw_set_invalid_parameter_handler
  12: mingw_set_invalid_parameter_handler
  13: mingw_set_invalid_parameter_handler
  14: mingw_set_invalid_parameter_handler
  15: rustdoc::clean::get_auto_traits_with_def_id
             at librustdoc\clean/auto_trait.rs:21
             at librustdoc\clean/mod.rs:3372
  16: rustdoc::clean::inline::build_impls
             at librustdoc\clean/inline.rs:246
  17: rustdoc::clean::build_deref_target_impls
             at librustdoc\clean/mod.rs:3431
  18: <rustdoc::doctree::Impl as rustdoc::clean::Clean<alloc::vec::Vec<rustdoc::clean::Item>>>::clean
             at librustdoc\clean/mod.rs:3384
  19: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
             at librustdoc\clean/mod.rs:541
             at C:\projects\rust\src\libcore\ops/function.rs:271
             at C:\projects\rust\src\libcore/option.rs:404
             at C:\projects\rust\src\libcore\iter/mod.rs:2448
             at C:\projects\rust\src\liballoc/vec.rs:1906
             at C:\projects\rust\src\liballoc/vec.rs:1803
             at C:\projects\rust\src\liballoc/vec.rs:1767
             at librustdoc\clean/mod.rs:541
  20: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::spec_extend
             at librustdoc\clean/mod.rs:536
             at C:\projects\rust\src\libcore\ops/function.rs:271
             at C:\projects\rust\src\libcore/option.rs:404
             at C:\projects\rust\src\libcore\iter/mod.rs:1311
             at C:\projects\rust\src\liballoc/vec.rs:1829
  21: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
             at C:\projects\rust\src\liballoc/vec.rs:1767
             at librustdoc\clean/mod.rs:536
  22: <rustdoc::visit_ast::RustdocVisitor<'a, 'tcx, 'rcx> as rustdoc::clean::Clean<rustdoc::clean::Crate>>::clean
             at librustdoc\clean/mod.rs:166
  23: rustdoc::core::run_core::{{closure}}
             at librustdoc/core.rs:270
  24: rustc::ty::context::TyCtxt::create_and_enter
             at C:\projects\rust\src\librustc_driver/driver.rs:1094
             at C:\projects\rust\src\librustc\ty/context.rs:1573
             at C:\projects\rust\src\libstd\thread/local.rs:377
             at C:\projects\rust\src\libstd\thread/local.rs:288
             at C:\projects\rust\src\librustc\ty/context.rs:1570
             at C:\projects\rust\src\librustc\ty/context.rs:1557
             at C:\projects\rust\src\libstd\thread/local.rs:377
             at C:\projects\rust\src\libstd\thread/local.rs:288
             at C:\projects\rust\src\librustc\ty/context.rs:1554
             at C:\projects\rust\src\librustc\ty/context.rs:1197
  25: rustdoc::core::run_core
             at C:\projects\rust\src\librustc_driver/driver.rs:1007
             at librustdoc/core.rs:217

error: internal compiler error: unexpected panic
