
  = note: ld.lld: error: undefined symbol: anon.26f7d47336e30322871c53dc21221119.123.llvm.9120153389878088914
          >>> referenced by compiler\rustc_codegen_llvm\src/declare.rs:123
          >>>               librustc_codegen_llvm-d0298bfe4a8d31a6.rlib(rustc_codegen_llvm-d0298bfe4a8d31a6.4tgytcm0fnpf9ska.rcgu.o):(rustc_codegen_ssa::base::maybe_create_entry_wrapper::<rustc_codegen_llvm::builder::Builder>)
          >>> referenced by compiler\rustc_codegen_llvm\src/declare.rs:76
          >>>               librustc_codegen_llvm-d0298bfe4a8d31a6.rlib(rustc_codegen_llvm-d0298bfe4a8d31a6.4tgytcm0fnpf9ska.rcgu.o):(rustc_codegen_ssa::base::maybe_create_entry_wrapper::<rustc_codegen_llvm::builder::Builder>)

          ld.lld: error: undefined symbol: <core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rustc_metadata::rmeta::Lazy<[rustc_ast::ast::Attribute], usize>>::decode<(rustc_metadata::creader::CrateMetadataRef, &rustc_session::session::Session)>::{closure#0}> as core::iter::traits::iterator::Iterator>::try_fold::<(), core::iter::traits::iterator::Iterator::find::check<rustc_ast::ast::Attribute, <rustc_resolve::Resolver>::legacy_const_generic_args::{closure#0}::{closure#0}>::{closure#0}, core::ops::control_flow::ControlFlow<rustc_ast::ast::Attribute>>
          >>> referenced by C:\msys64\home\we\rust\library\core\src\iter\traits/iterator.rs:2463
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.38k7y9bat2xg6222.rcgu.o):(<rustc_resolve::Resolver>::legacy_const_generic_args::{closure#0})

          ld.lld: error: undefined symbol: core::ptr::drop_in_place::<alloc::boxed::Box<rustc_ast::ast::Expr>> (.llvm.12350409468421822947)
          >>> referenced by C:\msys64\home\we\rust\library\core\src\ptr/mod.rs:0
          >>>               librustc_codegen_ssa-d1f61af6eec07775.rlib(rustc_codegen_ssa-d1f61af6eec07775.tftlaxklz1dark3.rcgu.o):(core::ptr::drop_in_place::<rustc_ast::ast::ExprKind>)
          collect2.exe: error: ld returned 1 exit status


error: could not compile `rustc_driver` due to previous error
