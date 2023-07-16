plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.84
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
thread 'rustc' panicked at 'invalid enum variant tag while decoding `TyKind`, expected 0..27', /checkout/compiler/rustc_type_ir/src/sty.rs:766:18
stack backtrace:
   0:     0x7eff4b503512 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h76efda83803f7c88
   1:     0x7eff4b571658 - core::fmt::write::h0cd9f5419c66d611
   2:     0x7eff4b4f3fe1 - std::io::Write::write_fmt::h115d78592252bf9d
   3:     0x7eff4b5032d5 - std::sys_common::backtrace::print::hd9ebfd55eb3388eb
   4:     0x7eff4b5065e7 - std::panicking::default_hook::{{closure}}::h98850a6457a8782d
   5:     0x7eff4b506345 - std::panicking::default_hook::hef750a0d73d5e8e9
   6:     0x7eff4bee6f14 - rustc_driver[ab3ba77cd7534cfb]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7eff4b506ef3 - std::panicking::rust_panic_with_hook::ha62dd9a4e901383e
   8:     0x7eff4b506c27 - std::panicking::begin_panic_handler::{{closure}}::h30577589484d4d4f
   9:     0x7eff4b503abc - std::sys_common::backtrace::__rust_end_short_backtrace::h8641e565b807163f
  10:     0x7eff4b5068f2 - rust_begin_unwind
  11:     0x7eff4b4b8023 - core::panicking::panic_fmt::ha29375e42f9e82fb
  12:     0x7eff4e1cd521 - core[867cfca19013d5a]::panicking::panic_display::<alloc[6d250edaf69a7784]::string::String>
  13:     0x7eff4e1dd7ce - <rustc_middle[4f81a7ac8c79b4c8]::ty::Ty as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode
  14:     0x7eff4e2668ef - <rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext as rustc_type_ir[9ccd8fdedb26e56b]::codec::TyDecoder>::cached_ty_for_shorthand::<<rustc_middle[4f81a7ac8c79b4c8]::ty::Ty as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode::{closure#0}>
  15:     0x7eff4e1dd380 - <rustc_middle[4f81a7ac8c79b4c8]::ty::Ty as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode
  16:     0x7eff4e1ddd0c - <&rustc_middle[4f81a7ac8c79b4c8]::ty::list::List<rustc_middle[4f81a7ac8c79b4c8]::ty::subst::GenericArg> as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode
  17:     0x7eff4e1dd5c7 - <rustc_middle[4f81a7ac8c79b4c8]::ty::Ty as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode
  18:     0x7eff4e2668ef - <rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext as rustc_type_ir[9ccd8fdedb26e56b]::codec::TyDecoder>::cached_ty_for_shorthand::<<rustc_middle[4f81a7ac8c79b4c8]::ty::Ty as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode::{closure#0}>
  19:     0x7eff4e1dd380 - <rustc_middle[4f81a7ac8c79b4c8]::ty::Ty as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode
  20:     0x7eff4e205cd3 - <rustc_metadata[48d6e4923091dbdd]::rmeta::LazyValue<rustc_middle[4f81a7ac8c79b4c8]::ty::Ty>>::decode::<(rustc_metadata[48d6e4923091dbdd]::creader::CrateMetadataRef, rustc_middle[4f81a7ac8c79b4c8]::ty::context::TyCtxt)>
  21:     0x7eff4e16024a - rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::cstore_impl::provide_extern::type_of
  22:     0x7eff4dc41715 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<rustc_span[84eb59eb28be3601]::def_id::DefId, rustc_middle[4f81a7ac8c79b4c8]::ty::Ty>>
  23:     0x7eff4dd833c8 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::type_of, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  24:     0x7eff4d903bae - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::type_of
  25:     0x7eff4c7c2865 - <dyn rustc_hir_analysis[310abef0984c523]::astconv::AstConv>::ast_path_to_ty
  26:     0x7eff4c7d1c60 - <dyn rustc_hir_analysis[310abef0984c523]::astconv::AstConv>::res_to_ty
  27:     0x7eff4c7e83e3 - <dyn rustc_hir_analysis[310abef0984c523]::astconv::AstConv>::ast_ty_to_ty_inner
  28:     0x7eff4c764733 - <core[867cfca19013d5a]::iter::adapters::map::Map<core[867cfca19013d5a]::iter::adapters::enumerate::Enumerate<core[867cfca19013d5a]::slice::iter::Iter<rustc_hir[3b98b05658f9e115]::hir::Ty>>, <dyn rustc_hir_analysis[310abef0984c523]::astconv::AstConv>::ty_of_fn::{closure#0}::{closure#0}> as core[867cfca19013d5a]::iter::traits::iterator::Iterator>::fold::<(), core[867cfca19013d5a]::iter::traits::iterator::Iterator::for_each::call<rustc_middle[4f81a7ac8c79b4c8]::ty::Ty, <alloc[6d250edaf69a7784]::vec::Vec<rustc_middle[4f81a7ac8c79b4c8]::ty::Ty>>::extend_trusted<core[867cfca19013d5a]::iter::adapters::map::Map<core[867cfca19013d5a]::iter::adapters::enumerate::Enumerate<core[867cfca19013d5a]::slice::iter::Iter<rustc_hir[3b98b05658f9e115]::hir::Ty>>, <dyn rustc_hir_analysis[310abef0984c523]::astconv::AstConv>::ty_of_fn::{closure#0}::{closure#0}>>::{closure#0}>::{closure#0}>
  29:     0x7eff4c7fca53 - <alloc[6d250edaf69a7784]::vec::Vec<rustc_middle[4f81a7ac8c79b4c8]::ty::Ty> as alloc[6d250edaf69a7784]::vec::spec_from_iter::SpecFromIter<rustc_middle[4f81a7ac8c79b4c8]::ty::Ty, core[867cfca19013d5a]::iter::adapters::map::Map<core[867cfca19013d5a]::iter::adapters::enumerate::Enumerate<core[867cfca19013d5a]::slice::iter::Iter<rustc_hir[3b98b05658f9e115]::hir::Ty>>, <dyn rustc_hir_analysis[310abef0984c523]::astconv::AstConv>::ty_of_fn::{closure#0}::{closure#0}>>>::from_iter
  30:     0x7eff4c7e9eea - <dyn rustc_hir_analysis[310abef0984c523]::astconv::AstConv>::ty_of_fn
  31:     0x7eff4c75a5f2 - rustc_hir_analysis[310abef0984c523]::collect::fn_sig
  32:     0x7eff4dc3a950 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<rustc_span[84eb59eb28be3601]::def_id::DefId, rustc_middle[4f81a7ac8c79b4c8]::ty::sty::Binder<rustc_middle[4f81a7ac8c79b4c8]::ty::sty::FnSig>>>
  33:     0x7eff4dd82ec5 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::fn_sig, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  34:     0x7eff4d92138c - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::fn_sig
  35:     0x7eff4c60a385 - <rustc_privacy[d80f65a39f38abc1]::DefIdVisitorSkeleton<rustc_privacy[d80f65a39f38abc1]::ReachEverythingInTheInterfaceVisitor> as rustc_middle[4f81a7ac8c79b4c8]::ty::visit::TypeVisitor>::visit_ty
  36:     0x7eff4c60e6e0 - <rustc_privacy[d80f65a39f38abc1]::ReachEverythingInTheInterfaceVisitor>::ty
  37:     0x7eff4c60cb1b - <rustc_privacy[d80f65a39f38abc1]::EmbargoVisitor as rustc_hir[3b98b05658f9e115]::intravisit::Visitor>::visit_item
  38:     0x7eff4c62187d - rustc_hir[3b98b05658f9e115]::intravisit::walk_item::<rustc_privacy[d80f65a39f38abc1]::EmbargoVisitor>
  39:     0x7eff4c60cc5e - <rustc_privacy[d80f65a39f38abc1]::EmbargoVisitor as rustc_hir[3b98b05658f9e115]::intravisit::Visitor>::visit_item
  40:     0x7eff4c62187d - rustc_hir[3b98b05658f9e115]::intravisit::walk_item::<rustc_privacy[d80f65a39f38abc1]::EmbargoVisitor>
  41:     0x7eff4c60cc5e - <rustc_privacy[d80f65a39f38abc1]::EmbargoVisitor as rustc_hir[3b98b05658f9e115]::intravisit::Visitor>::visit_item
  42:     0x7eff4c61e6ca - rustc_hir[3b98b05658f9e115]::intravisit::walk_mod::<rustc_privacy[d80f65a39f38abc1]::EmbargoVisitor>
  43:     0x7eff4c615a6b - rustc_privacy[d80f65a39f38abc1]::effective_visibilities
  44:     0x7eff4dc681b2 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<(), &rustc_middle[4f81a7ac8c79b4c8]::middle::privacy::EffectiveVisibilities>>
  45:     0x7eff4dd560d0 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::effective_visibilities, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  46:     0x7eff4d931dea - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::effective_visibilities
  47:     0x7eff4cedbdca - rustc_passes[2183dd064de28808]::stability::check_unused_or_stable_features
  48:     0x7eff4c012570 - <rustc_session[26948ece6d29333b]::session::Session>::time::<(), rustc_interface[f9336dc587154158]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  49:     0x7eff4c00437d - std[65b05b99172e1c7e]::panic::catch_unwind::<core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[f9336dc587154158]::passes::analysis::{closure#0}::{closure#2}>, ()>
  50:     0x7eff4c0135b6 - <rustc_session[26948ece6d29333b]::session::Session>::time::<(), rustc_interface[f9336dc587154158]::passes::analysis::{closure#0}>
  51:     0x7eff4c033cb6 - rustc_interface[f9336dc587154158]::passes::analysis
  52:     0x7eff4dc5f215 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<(), core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>>
  53:     0x7eff4dd834d0 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::analysis, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  54:     0x7eff4d90488a - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::analysis
  55:     0x7eff4bf3e338 - <rustc_interface[f9336dc587154158]::passes::QueryContext>::enter::<rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  56:     0x7eff4bf4825e - <rustc_interface[f9336dc587154158]::interface::Compiler>::enter::<rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}::{closure#2}, core[867cfca19013d5a]::result::Result<core[867cfca19013d5a]::option::Option<rustc_interface[f9336dc587154158]::queries::Linker>, rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  57:     0x7eff4bee85d2 - rustc_span[84eb59eb28be3601]::with_source_map::<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  58:     0x7eff4bf48d58 - <scoped_tls[b64ea83672690cf8]::ScopedKey<rustc_span[84eb59eb28be3601]::SessionGlobals>>::set::<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  59:     0x7eff4bf05af0 - std[65b05b99172e1c7e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f9336dc587154158]::util::run_in_thread_pool_with_globals<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  60:     0x7eff4bf4a106 - std[65b05b99172e1c7e]::panicking::try::<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<<std[65b05b99172e1c7e]::thread::Builder>::spawn_unchecked_<rustc_interface[f9336dc587154158]::util::run_in_thread_pool_with_globals<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  61:     0x7eff4bef6b59 - <<std[65b05b99172e1c7e]::thread::Builder>::spawn_unchecked_<rustc_interface[f9336dc587154158]::util::run_in_thread_pool_with_globals<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#1} as core[867cfca19013d5a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  62:     0x7eff4b513e0e - std::sys::unix::thread::Thread::new::thread_start::h0d9480603bdbd4eb
  63:     0x7eff4b2a8b43 - <unknown>
  64:     0x7eff4b33aa00 - <unknown>
  65:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (9b7b51cad 2022-11-27) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=10000 -C debuginfo=0 -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_of] computing type of `core::option::Option`
#0 [type_of] computing type of `core::option::Option`
#1 [fn_sig] computing function signature of `int::udiv::__udivmodsi4`
#2 [effective_visibilities] checking effective visibilities
#3 [analysis] running analysis passes on this crate
error: could not compile `compiler_builtins`
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'invalid enum variant tag while decoding `TyKind`, expected 0..27', /checkout/compiler/rustc_type_ir/src/sty.rs:766:18
stack backtrace:
   0:     0x7f649a867512 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h76efda83803f7c88
   1:     0x7f649a8d5658 - core::fmt::write::h0cd9f5419c66d611
   2:     0x7f649a857fe1 - std::io::Write::write_fmt::h115d78592252bf9d
   3:     0x7f649a8672d5 - std::sys_common::backtrace::print::hd9ebfd55eb3388eb
   4:     0x7f649a86a5e7 - std::panicking::default_hook::{{closure}}::h98850a6457a8782d
   5:     0x7f649a86a345 - std::panicking::default_hook::hef750a0d73d5e8e9
   6:     0x7f649b24af14 - rustc_driver[ab3ba77cd7534cfb]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f649a86aef3 - std::panicking::rust_panic_with_hook::ha62dd9a4e901383e
   8:     0x7f649a86ac27 - std::panicking::begin_panic_handler::{{closure}}::h30577589484d4d4f
   9:     0x7f649a867abc - std::sys_common::backtrace::__rust_end_short_backtrace::h8641e565b807163f
  10:     0x7f649a86a8f2 - rust_begin_unwind
  11:     0x7f649a81c023 - core::panicking::panic_fmt::ha29375e42f9e82fb
  12:     0x7f649d531521 - core[867cfca19013d5a]::panicking::panic_display::<alloc[6d250edaf69a7784]::string::String>
  13:     0x7f649d5417ce - <rustc_middle[4f81a7ac8c79b4c8]::ty::Ty as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode
  14:     0x7f649d5ca8ef - <rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext as rustc_type_ir[9ccd8fdedb26e56b]::codec::TyDecoder>::cached_ty_for_shorthand::<<rustc_middle[4f81a7ac8c79b4c8]::ty::Ty as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode::{closure#0}>
  15:     0x7f649d541380 - <rustc_middle[4f81a7ac8c79b4c8]::ty::Ty as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode
  16:     0x7f649d541d0c - <&rustc_middle[4f81a7ac8c79b4c8]::ty::list::List<rustc_middle[4f81a7ac8c79b4c8]::ty::subst::GenericArg> as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode
  17:     0x7f649d5415c7 - <rustc_middle[4f81a7ac8c79b4c8]::ty::Ty as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode
  18:     0x7f649d5ca8ef - <rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext as rustc_type_ir[9ccd8fdedb26e56b]::codec::TyDecoder>::cached_ty_for_shorthand::<<rustc_middle[4f81a7ac8c79b4c8]::ty::Ty as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode::{closure#0}>
  19:     0x7f649d541380 - <rustc_middle[4f81a7ac8c79b4c8]::ty::Ty as rustc_serialize[a271f74fabc451c9]::serialize::Decodable<rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::DecodeContext>>::decode
  20:     0x7f649d569cd3 - <rustc_metadata[48d6e4923091dbdd]::rmeta::LazyValue<rustc_middle[4f81a7ac8c79b4c8]::ty::Ty>>::decode::<(rustc_metadata[48d6e4923091dbdd]::creader::CrateMetadataRef, rustc_middle[4f81a7ac8c79b4c8]::ty::context::TyCtxt)>
  21:     0x7f649d4c424a - rustc_metadata[48d6e4923091dbdd]::rmeta::decoder::cstore_impl::provide_extern::type_of
  22:     0x7f649cfa5715 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<rustc_span[84eb59eb28be3601]::def_id::DefId, rustc_middle[4f81a7ac8c79b4c8]::ty::Ty>>
  23:     0x7f649d0e73c8 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::type_of, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  24:     0x7f649cc67bae - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::type_of
  25:     0x7f649bb26865 - <dyn rustc_hir_analysis[310abef0984c523]::astconv::AstConv>::ast_path_to_ty
  26:     0x7f649bb35c60 - <dyn rustc_hir_analysis[310abef0984c523]::astconv::AstConv>::res_to_ty
  27:     0x7f649bb4c3e3 - <dyn rustc_hir_analysis[310abef0984c523]::astconv::AstConv>::ast_ty_to_ty_inner
  28:     0x7f649b9b8777 - rustc_hir_analysis[310abef0984c523]::collect::type_of::type_of
  29:     0x7f649cfa5715 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<rustc_span[84eb59eb28be3601]::def_id::DefId, rustc_middle[4f81a7ac8c79b4c8]::ty::Ty>>
  30:     0x7f649d0e73c8 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::type_of, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  31:     0x7f649cc67bae - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::type_of
  32:     0x7f649ba5aaf1 - rustc_hir_analysis[310abef0984c523]::outlives::implicit_infer::infer_predicates
  33:     0x7f649bbac8bf - rustc_hir_analysis[310abef0984c523]::outlives::inferred_outlives_crate
  34:     0x7f649cf831ca - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::ArenaCache<(), rustc_middle[4f81a7ac8c79b4c8]::ty::CratePredicatesMap>>
  35:     0x7f649d0c48a0 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::inferred_outlives_crate, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  36:     0x7f649cc7ec4a - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::inferred_outlives_crate
  37:     0x7f649bbabfd4 - rustc_hir_analysis[310abef0984c523]::outlives::inferred_outlives_of
  38:     0x7f649cfbc803 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<rustc_span[84eb59eb28be3601]::def_id::DefId, &[(rustc_middle[4f81a7ac8c79b4c8]::ty::Predicate, rustc_span[84eb59eb28be3601]::span_encoding::Span)]>>
  39:     0x7f649d0b204d - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::inferred_outlives_of, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  40:     0x7f649cc7842e - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::inferred_outlives_of
  41:     0x7f649babf7cd - rustc_hir_analysis[310abef0984c523]::collect::predicates_defined_on
  42:     0x7f649cfaa570 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<rustc_span[84eb59eb28be3601]::def_id::DefId, rustc_middle[4f81a7ac8c79b4c8]::ty::generics::GenericPredicates>>
  43:     0x7f649d0b627e - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::predicates_defined_on, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  44:     0x7f649cc7708c - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::predicates_defined_on
  45:     0x7f649ba8a295 - rustc_hir_analysis[310abef0984c523]::collect::predicates_of::predicates_of
  46:     0x7f649cfaa570 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<rustc_span[84eb59eb28be3601]::def_id::DefId, rustc_middle[4f81a7ac8c79b4c8]::ty::generics::GenericPredicates>>
  47:     0x7f649d0954ca - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::predicates_of, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  48:     0x7f649cc69bec - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::predicates_of
  49:     0x7f649b9723ef - <rustc_privacy[d80f65a39f38abc1]::ReachEverythingInTheInterfaceVisitor>::predicates
  50:     0x7f649b970d68 - <rustc_privacy[d80f65a39f38abc1]::EmbargoVisitor as rustc_hir[3b98b05658f9e115]::intravisit::Visitor>::visit_item
  51:     0x7f649b98587d - rustc_hir[3b98b05658f9e115]::intravisit::walk_item::<rustc_privacy[d80f65a39f38abc1]::EmbargoVisitor>
  52:     0x7f649b970c5e - <rustc_privacy[d80f65a39f38abc1]::EmbargoVisitor as rustc_hir[3b98b05658f9e115]::intravisit::Visitor>::visit_item
  53:     0x7f649b9826ca - rustc_hir[3b98b05658f9e115]::intravisit::walk_mod::<rustc_privacy[d80f65a39f38abc1]::EmbargoVisitor>
  54:     0x7f649b979a6b - rustc_privacy[d80f65a39f38abc1]::effective_visibilities
  55:     0x7f649cfcc1b2 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<(), &rustc_middle[4f81a7ac8c79b4c8]::middle::privacy::EffectiveVisibilities>>
  56:     0x7f649d0ba0d0 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::effective_visibilities, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  57:     0x7f649cc95dea - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::effective_visibilities
  58:     0x7f649c23fdca - rustc_passes[2183dd064de28808]::stability::check_unused_or_stable_features
  59:     0x7f649b376570 - <rustc_session[26948ece6d29333b]::session::Session>::time::<(), rustc_interface[f9336dc587154158]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  60:     0x7f649b36837d - std[65b05b99172e1c7e]::panic::catch_unwind::<core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[f9336dc587154158]::passes::analysis::{closure#0}::{closure#2}>, ()>
  61:     0x7f649b3775b6 - <rustc_session[26948ece6d29333b]::session::Session>::time::<(), rustc_interface[f9336dc587154158]::passes::analysis::{closure#0}>
  62:     0x7f649b397cb6 - rustc_interface[f9336dc587154158]::passes::analysis
  63:     0x7f649cfc3215 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<(), core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>>
  64:     0x7f649d0e74d0 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::analysis, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  65:     0x7f649cc6888a - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::analysis
  66:     0x7f649b2a2338 - <rustc_interface[f9336dc587154158]::passes::QueryContext>::enter::<rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  67:     0x7f649b2ac25e - <rustc_interface[f9336dc587154158]::interface::Compiler>::enter::<rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}::{closure#2}, core[867cfca19013d5a]::result::Result<core[867cfca19013d5a]::option::Option<rustc_interface[f9336dc587154158]::queries::Linker>, rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  68:     0x7f649b24c5d2 - rustc_span[84eb59eb28be3601]::with_source_map::<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  69:     0x7f649b2acd58 - <scoped_tls[b64ea83672690cf8]::ScopedKey<rustc_span[84eb59eb28be3601]::SessionGlobals>>::set::<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  70:     0x7f649b269af0 - std[65b05b99172e1c7e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f9336dc587154158]::util::run_in_thread_pool_with_globals<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  71:     0x7f649b2ae106 - std[65b05b99172e1c7e]::panicking::try::<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<<std[65b05b99172e1c7e]::thread::Builder>::spawn_unchecked_<rustc_interface[f9336dc587154158]::util::run_in_thread_pool_with_globals<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  72:     0x7f649b25ab59 - <<std[65b05b99172e1c7e]::thread::Builder>::spawn_unchecked_<rustc_interface[f9336dc587154158]::util::run_in_thread_pool_with_globals<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#1} as core[867cfca19013d5a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  73:     0x7f649a877e0e - std::sys::unix::thread::Thread::new::thread_start::h0d9480603bdbd4eb
  74:     0x7f649a60cb43 - <unknown>
  75:     0x7f649a69ea00 - <unknown>
  76:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (9b7b51cad 2022-11-27) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_of] computing type of `core::option::Option`
#0 [type_of] computing type of `core::option::Option`
#1 [type_of] computing type of `unix::linux_like::linux::gnu::b64::x86_64::sigaction::sa_restorer`
#2 [inferred_outlives_crate] computing the inferred outlives predicates for items in this crate
#3 [inferred_outlives_of] computing inferred outlives predicates of `unix::DIR`
#4 [predicates_defined_on] computing predicates of `unix::DIR`
#5 [predicates_of] computing predicates of `unix::DIR`
#6 [effective_visibilities] checking effective visibilities
#7 [analysis] running analysis passes on this crate
error: could not compile `libc`
Build completed unsuccessfully in 0:03:44
