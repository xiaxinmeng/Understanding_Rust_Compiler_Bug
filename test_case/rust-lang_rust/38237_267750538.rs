
thread 'rustc' panicked at 'assertion failed: !self.is_proc_macro(item_id)', C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\librustc_metadata\decoder.rs:523
stack backtrace:
   0:      0x7fef26ced8a - std::panicking::Location::line::he2106c63a7bda898
   1:      0x7fef26ce214 - std::panicking::Location::line::he2106c63a7bda898
   2:      0x7fef26d1c6d - std::panicking::rust_panic_with_hook::h6704edd5f66c8031
   3:      0x7fef29c51f9 - <unknown>
   4:      0x7fef2a35ed0 - rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::entry::hbc7aed61dbec21a6
   5:      0x7fef2a38689 - rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::get_visibility::h807eba63c03b81b9
   6:      0x7fef2a38b9c - rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::get_impl_trait::h3f841a5c9799a42e
   7:      0x7fef2a41d79 - rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore<'tcx> for rustc_metadata::cstore::CStore>::impl_trait_ref::ha1d833d9d46c191c
   8:      0x7fee7efdbd7 - rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::impl_trait_ref::h8c67f9358bd74507
   9:      0x7feea7c7453 - rustdoc::clean::inline::build_impl::hf58f63c15715029c
  10:      0x7feea7c6d83 - rustdoc::clean::inline::build_impls::h14e91e677e8f7d94
  11:      0x7feea7ee38f - <rustdoc::doctree::Impl as rustdoc::clean::Clean<collections::vec::Vec<rustdoc::clean::Item>>>::clean::hcaddb48ccf539d21
  12:      0x7feea7ecce1 - <rustdoc::doctree::Impl as rustdoc::clean::Clean<collections::vec::Vec<rustdoc::clean::Item>>>::clean::hcaddb48ccf539d21
  13:      0x7feea7d020f - <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean::h2fa8f90404b95ae3
  14:      0x7feea7cbfa9 - <rustdoc::visit_ast::RustdocVisitor<'a, 'tcx> as rustdoc::clean::Clean<rustdoc::clean::Crate>>::clean::hfacf6af3022b4e16
  15:      0x7feea7f92a7 - rustdoc::core::run_core::h00b2d9e4204dfeaf
  16:      0x7feea71b20f - <unknown>
  17:      0x7feea772a7f - <unknown>
  18:      0x7feea7f66fc - rustdoc::core::run_core::h00b2d9e4204dfeaf
  19:      0x7feea73e6dd - <unknown>
  20:      0x7fef26d4621 - _rust_maybe_catch_panic
  21:      0x7feea76352b - <unknown>
  22:      0x7fef26cc34e - std::sys::imp::thread::Thread::new::h187d5ea36be54766
  23:         0x774659cc - BaseThreadInitThunk

error: Could not document `bar`.
