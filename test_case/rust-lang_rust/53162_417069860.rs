
 Documenting rustc_codegen_llvm v0.0.0 (file:///home/misdreavus/git/rust-builds/src/librustc_codegen_llvm)                                             error[E0275]: overflow evaluating the requirement `std::boxed::Box<std::vec::Vec<syntax::ast::Attribute>>: std::marker::Sync`                            |                                                                                                                                                      = help: consider adding a `#![recursion_limit="128"]` attribute to your crate                                                                          = note: required because it appears within the type `std::option::Option<std::boxed::Box<std::vec::Vec<syntax::ast::Attribute>>>`                      = note: required because it appears within the type `syntax::ThinVec<syntax::ast::Attribute>`                                                          = note: required because it appears within the type `syntax::ast::FieldPat`
  = note: required because it appears within the type `syntax::source_map::Spanned<syntax::ast::FieldPat>`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::source_map::Spanned<syntax::ast::FieldP
at>>`                                                                                                                                                    = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::source_map::Spanned<syntax::ast::FieldPat>>`
  = note: required because it appears within the type `std::vec::Vec<syntax::source_map::Spanned<syntax::ast::FieldPat>>`                                = note: required because it appears within the type `syntax::ast::PatKind`                                                                             = note: required because it appears within the type `syntax::ast::Pat`                                                                                 = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Pat>`                               = note: required because it appears within the type `std::boxed::Box<syntax::ast::Pat>`
  = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Pat>`
  = note: required because it appears within the type `syntax::ast::Local`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Local>`
  = note: required because it appears within the type `std::boxed::Box<syntax::ast::Local>`
  = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Local>`
  = note: required because it appears within the type `syntax::ast::StmtKind`
  = note: required because it appears within the type `syntax::ast::Stmt`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Stmt>`                              = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Stmt>`                                                        = note: required because it appears within the type `std::vec::Vec<syntax::ast::Stmt>`                                                                 = note: required because it appears within the type `syntax::ast::Block`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Block>`
  = note: required because it appears within the type `std::boxed::Box<syntax::ast::Block>`
  = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Block>`                                                               = note: required because it appears within the type `syntax::ast::ExprKind`
  = note: required because it appears within the type `syntax::ast::Expr`                                                                                = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Expr>`                              = note: required because it appears within the type `std::boxed::Box<syntax::ast::Expr>`                                                               = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Expr>`                                                                = note: required because it appears within the type `syntax::ast::AnonConst`
  = note: required because it appears within the type `syntax::ast::TyKind`
  = note: required because it appears within the type `syntax::ast::Ty`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Ty>`
  = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
  = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
  = note: required because it appears within the type `syntax::ast::GenericArg`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::GenericArg>`
  = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::GenericArg>`
  = note: required because it appears within the type `std::vec::Vec<syntax::ast::GenericArg>`
  = note: required because it appears within the type `syntax::ast::AngleBracketedArgs`
  = note: required because it appears within the type `syntax::ast::GenericArgs`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::GenericArgs>`
  = note: required because it appears within the type `std::boxed::Box<syntax::ast::GenericArgs>`
  = note: required because it appears within the type `syntax::ptr::P<syntax::ast::GenericArgs>`
  = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::GenericArgs>>`
  = note: required because it appears within the type `syntax::ast::PathSegment`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::PathSegment>`
  = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::PathSegment>`
  = note: required because it appears within the type `std::vec::Vec<syntax::ast::PathSegment>`
  = note: required because it appears within the type `syntax::ast::Path`
  = note: required because it appears within the type `syntax::ast::MetaItem`
  = note: required because it appears within the type `std::option::Option<syntax::ast::MetaItem>`
  = note: required because it appears within the type `rustc::middle::cstore::NativeLibrary`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc::middle::cstore::NativeLibrary>`
  = note: required because it appears within the type `alloc::raw_vec::RawVec<rustc::middle::cstore::NativeLibrary>`
  = note: required because it appears within the type `std::vec::Vec<rustc::middle::cstore::NativeLibrary>`
  = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<std::vec::Vec<rustc::middle::cstore::NativeLibrar
y>>`
  = note: required because it appears within the type `(rustc::hir::def_id::CrateNum, std::sync::Arc<std::vec::Vec<rustc::middle::cstore::NativeLibrary
>>)`
  = note: required because it appears within the type `std::marker::PhantomData<(rustc::hir::def_id::CrateNum, std::sync::Arc<std::vec::Vec<rustc::midd
le::cstore::NativeLibrary>>)>`
  = note: required because it appears within the type `std::collections::hash::table::RawTable<rustc::hir::def_id::CrateNum, std::sync::Arc<std::vec::V
ec<rustc::middle::cstore::NativeLibrary>>>`
  = note: required because it appears within the type `std::collections::HashMap<rustc::hir::def_id::CrateNum, std::sync::Arc<std::vec::Vec<rustc::midd
le::cstore::NativeLibrary>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
  = note: required because it appears within the type `CrateInfo`
  = note: required because it appears within the type `CodegenResults`

error: Could not document `rustc_codegen_llvm`.
