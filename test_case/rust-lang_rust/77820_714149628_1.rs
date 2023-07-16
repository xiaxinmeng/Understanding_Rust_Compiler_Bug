
   8: rustc_middle::util::bug::bug_fmt
             at ./compiler/rustc_middle/src/util/bug.rs:14:5
   9: rustc_middle::hir::map::Map::get::{{closure}}
             at ./compiler/rustc_middle/src/hir/map/mod.rs:477:41
  10: core::option::Option<T>::unwrap_or_else
             at ./library/core/src/option.rs:427:21
  11: rustc_middle::hir::map::Map::get
             at ./compiler/rustc_middle/src/hir/map/mod.rs:477:9
  12: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::opt_item_name::{{closure}}
             at ./compiler/rustc_middle/src/ty/mod.rs:2796:32
  13: core::option::Option<T>::and_then
             at ./library/core/src/option.rs:662:24
  14: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::opt_item_name
             at ./compiler/rustc_middle/src/ty/mod.rs:2794:9
  15: rustdoc::clean::types::Item::from_def_id_and_kind
             at ./src/librustdoc/clean/types.rs:127:19
  16: rustdoc::clean::types::Item::from_hir_id_and_kind
             at ./src/librustdoc/clean/types.rs:118:9
  17: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::types::Item>>::clean
             at ./src/librustdoc/clean/mod.rs:265:33
  18: rustdoc::clean::utils::krate
             at ./src/librustdoc/clean/utils.rs:44:22
