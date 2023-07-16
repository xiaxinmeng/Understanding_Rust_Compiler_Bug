
error[E0275]: overflow evaluating the requirement `alloc::raw_vec::RawVec<(rustc_ast::ast::UseTree, rustc_ast::ast::NodeId)>: std::marker::Sync`
  |
  = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`rustc_plugin_impl`)
  = note: required because it appears within the type `std::vec::Vec<(rustc_ast::ast::UseTree, rustc_ast::ast::NodeId)>`
  = note: required because it appears within the type `rustc_ast::ast::UseTreeKind`
  ... <lots more lines like that> ...
