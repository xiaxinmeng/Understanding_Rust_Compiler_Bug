backtrace

     Running `target/debug/miri --sysroot /Users/ouz/Library/Caches/org.rust-lang.miri/HOST tabula.rs`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(Obligation(predicate=Binder(TraitPredicate(<&S as std::marker::Sized>, polarity:Positive), []), depth=0))`,
 right: `None`', compiler/rustc_traits/src/normalize_erasing_regions.rs:40:17
stack backtrace:
   0:        0x104436658 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h502b9907255a9020
   1:        0x1044c1c58 - core::fmt::write::h1312fa7bb85e0d03
   2:        0x104467b54 - std::io::Write::write_fmt::hdc03145f6f1e9dcd
   3:        0x1044364ac - std::sys_common::backtrace::print::h3f2486f04aea89c9
   4:        0x10445fc70 - std::panicking::default_hook::{{closure}}::haa37fd6bee37c759
   5:        0x10445fa18 - std::panicking::default_hook::hc228b3d9e7765ab2
   6:        0x10cf9e394 - rustc_driver[6168a678ad3ae7e8]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x104460158 - std::panicking::rust_panic_with_hook::h780d45e68728c0ed
   8:        0x104454b08 - std::panicking::begin_panic_handler::{{closure}}::hc8833628902d6139
   9:        0x104454a48 - std::sys_common::backtrace::__rust_end_short_backtrace::h12758624bd828a06
  10:        0x10445fcd0 - _rust_begin_unwind
  11:        0x1044e2c68 - core::panicking::panic_fmt::h855bf55d5a000e71
  12:        0x1044cc4bc - core::panicking::assert_failed_inner::h296340acbf3dd196
  13:        0x1111fe884 - core[e78181ecd1ceeb9c]::panicking::assert_failed::<core[e78181ecd1ceeb9c]::option::Option<&rustc_infer[74bbb1a65a4d2108]::traits::Obligation<rustc_middle[bdd54a4263b5ba61]::ty::Predicate>>, core[e78181ecd1ceeb9c]::option::Option<&rustc_infer[74bbb1a65a4d2108]::traits::Obligation<rustc_middle[bdd54a4263b5ba61]::ty::Predicate>>>
  14:        0x10fd67c58 - <rustc_infer[74bbb1a65a4d2108]::infer::InferCtxtBuilder>::enter::<core[e78181ecd1ceeb9c]::result::Result<rustc_middle[bdd54a4263b5ba61]::ty::subst::GenericArg, rustc_middle[bdd54a4263b5ba61]::traits::query::NoSolution>, rustc_traits[82be05223e04c67b]::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle[bdd54a4263b5ba61]::ty::subst::GenericArg>::{closure#0}>
  15:        0x10fce0924 - rustc_traits[82be05223e04c67b]::normalize_erasing_regions::try_normalize_after_erasing_regions::<rustc_middle[bdd54a4263b5ba61]::ty::subst::GenericArg>
  16:        0x10fd962d8 - <rustc_traits[82be05223e04c67b]::normalize_erasing_regions::provide::{closure#0} as core[e78181ecd1ceeb9c]::ops::function::FnOnce<(rustc_middle[bdd54a4263b5ba61]::ty::context::TyCtxt, rustc_middle[bdd54a4263b5ba61]::ty::ParamEnvAnd<rustc_middle[bdd54a4263b5ba61]::ty::subst::GenericArg>)>>::call_once
  17:        0x11008f42c - rustc_query_system[f60b1df5a59e085c]::query::plumbing::get_query::<rustc_query_impl[3a679ff423363666]::queries::try_normalize_generic_arg_after_erasing_regions, rustc_query_impl[3a679ff423363666]::plumbing::QueryCtxt>
  18:        0x1102d9a00 - <rustc_query_impl[3a679ff423363666]::Queries as rustc_middle[bdd54a4263b5ba61]::ty::query::QueryEngine>::try_normalize_generic_arg_after_erasing_regions
  19:        0x110d18af8 - <rustc_middle[bdd54a4263b5ba61]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>::normalize_generic_arg_after_erasing_regions
  20:        0x110d18500 - <rustc_middle[bdd54a4263b5ba61]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle[bdd54a4263b5ba61]::ty::fold::TypeFolder>::fold_ty
  21:        0x102d2ace4 - <F as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty::h24c13ced97ba123b
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_middle/src/ty/fold.rs:216:12
  22:        0x102e8b70c - rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::ty::Ty>::try_fold_with::he175f578d2a156c8
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_middle/src/ty/structural_impls.rs:987:9
  23:        0x102ca3e08 - rustc_middle::ty::util::fold_list::{{closure}}::hd7b28e864b662f36
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_middle/src/ty/util.rs:1219:61
  24:        0x102c9323c - core::iter::traits::iterator::Iterator::find_map::check::{{closure}}::ha5ba178cfdb7bd06
                               at /Users/ouz/Code/GitHub/rust/library/core/src/iter/traits/iterator.rs:2732:32
  25:        0x102dc25d0 - <core::iter::adapters::enumerate::Enumerate<I> as core::iter::traits::iterator::Iterator>::try_fold::enumerate::{{closure}}::h8edb430b2c4ccedf
                               at /Users/ouz/Code/GitHub/rust/library/core/src/iter/adapters/enumerate.rs:85:27
  26:        0x102e968a8 - core::iter::traits::iterator::Iterator::try_fold::h672ebcdc41a6745c
                               at /Users/ouz/Code/GitHub/rust/library/core/src/iter/traits/iterator.rs:2238:21
  27:        0x102dc1d28 - <core::iter::adapters::enumerate::Enumerate<I> as core::iter::traits::iterator::Iterator>::try_fold::hd05ce0fa767ed83e
                               at /Users/ouz/Code/GitHub/rust/library/core/src/iter/adapters/enumerate.rs:91:9
  28:        0x102dc35d4 - core::iter::traits::iterator::Iterator::find_map::hbf25e6d0208cbe41
                               at /Users/ouz/Code/GitHub/rust/library/core/src/iter/traits/iterator.rs:2738:9
  29:        0x102ca335c - rustc_middle::ty::util::fold_list::habc928cafe10c2ac
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_middle/src/ty/util.rs:1219:11
  30:        0x102e9b3cc - rustc_middle::ty::subst::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::list::List<rustc_middle::ty::Ty>>::try_fold_with::h6f1546f0de215026
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_middle/src/ty/subst.rs:488:18
  31:        0x102e21ab0 - rustc_middle::ty::sty::_DERIVE_rustc_middle_ty_fold_TypeFoldable_tcx_FOR_FnSig::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::ty::sty::FnSig>::try_fold_with::h9c0f72204c364afa
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_middle/src/ty/sty.rs:1215:22
  32:        0x102e2348c - rustc_middle::ty::fold::TypeFoldable::fold_with::h1d568ba0d9ba8a09
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_middle/src/ty/fold.rs:74:9
  33:        0x102ee8afc - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions::hc4f250aeda8b9d76
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:58:13
  34:        0x102ee9dec - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_late_bound_regions::hf602c6eb9e19a655
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:112:9
  35:        0x102f2c2a4 - rustc_const_eval::interpret::terminator::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::eval_terminator::h4af56fdce5c5c777
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_const_eval/src/interpret/terminator.rs:76:21
  36:        0x102f4a164 - rustc_const_eval::interpret::step::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::terminator::he6130a6d48b90b19
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_const_eval/src/interpret/step.rs:312:9
  37:        0x102c5d9ac - rustc_const_eval::interpret::step::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::step::ha0fc27499a484102
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_const_eval/src/interpret/step.rs:71:9
  38:        0x102c5d9ac - miri::eval::eval_entry::{{closure}}::h5810cc380430e7ca
                               at /Users/ouz/Code/GitHub/rust/src/tools/miri/src/eval.rs:341:29
  39:        0x102e6ea30 - miri::eval::eval_entry::hb3886ee040677382
                               at /Users/ouz/Code/GitHub/rust/src/tools/miri/src/eval.rs:335:38
  40:        0x102acc368 - <miri::MiriCompilerCalls as rustc_driver::Callbacks>::after_analysis::{{closure}}::h47dfbaeb064e3774
                               at /Users/ouz/Code/GitHub/rust/src/tools/miri/src/bin/miri.rs:78:40
  41:        0x102acbd6c - rustc_interface::passes::QueryContext::enter::{{closure}}::ha7d8c04dc6b60909
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_interface/src/passes.rs:787:42
  42:        0x102aca0ec - rustc_middle::ty::context::tls::enter_context::{{closure}}::hc17f98344f622bcf
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_middle/src/ty/context.rs:1930:50
  43:        0x102aca1ac - rustc_middle::ty::context::tls::set_tlv::h80e6da773dbd83e0
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_middle/src/ty/context.rs:1914:9
  44:        0x102aca0a4 - rustc_middle::ty::context::tls::enter_context::h696299c6691d92eb
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_middle/src/ty/context.rs:1930:9
  45:        0x102acbd20 - rustc_interface::passes::QueryContext::enter::hcdddf80d42aa3963
                               at /Users/ouz/Code/GitHub/rust/compiler/rustc_interface/src/passes.rs:787:9
  46:        0x102ac5cb0 - <miri::MiriCompilerCalls as rustc_driver::Callbacks>::after_analysis::hec847b24b098d72d
                               at /Users/ouz/Code/GitHub/rust/src/tools/miri/src/bin/miri.rs:61:9
  47:        0x10cf90810 - <rustc_interface[bd25737d3859b110]::interface::Compiler>::enter::<rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}::{closure#2}, core[e78181ecd1ceeb9c]::result::Result<core[e78181ecd1ceeb9c]::option::Option<rustc_interface[bd25737d3859b110]::queries::Linker>, rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>
  48:        0x10cf8f8e8 - rustc_span[dbe5a3cbc50f2bf]::with_source_map::<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_interface[bd25737d3859b110]::interface::create_compiler_and_run<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}>::{closure#1}>
  49:        0x10cf911ac - rustc_interface[bd25737d3859b110]::interface::create_compiler_and_run::<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}>
  50:        0x10cf7ed80 - <scoped_tls[e8ecb510089da108]::ScopedKey<rustc_span[dbe5a3cbc50f2bf]::SessionGlobals>>::set::<rustc_interface[bd25737d3859b110]::interface::run_compiler<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>
  51:        0x10cfacd6c - std[4c7b4fc7f8c44cca]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bd25737d3859b110]::util::run_in_thread_pool_with_globals<rustc_interface[bd25737d3859b110]::interface::run_compiler<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>
  52:        0x10cfa2b60 - std[4c7b4fc7f8c44cca]::panicking::try::<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, core[e78181ecd1ceeb9c]::panic::unwind_safe::AssertUnwindSafe<<std[4c7b4fc7f8c44cca]::thread::Builder>::spawn_unchecked_<rustc_interface[bd25737d3859b110]::util::run_in_thread_pool_with_globals<rustc_interface[bd25737d3859b110]::interface::run_compiler<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  53:        0x10cfa4060 - <<std[4c7b4fc7f8c44cca]::thread::Builder>::spawn_unchecked_<rustc_interface[bd25737d3859b110]::util::run_in_thread_pool_with_globals<rustc_interface[bd25737d3859b110]::interface::run_compiler<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>::{closure#1} as core[e78181ecd1ceeb9c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  54:        0x1044553a8 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3a51253c442e9fd2
  55:        0x10445cf80 - std::sys::unix::thread::Thread::new::thread_start::hca7e919e365a1d36
  56:        0x1b385c26c - __pthread_deallocate

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-dev running on aarch64-apple-darwin

query stack during panic:
#0 [try_normalize_generic_arg_after_erasing_regions] normalizing `<[closure@tabula.rs:24:19: 24:22] as core::ops::function::FnOnce<(&S,)>>::Output`
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: VecMap([(OpaqueTypeKey { def_id: DefId(0:7 ~ tabula[475d]::Alias::{opaque#0}), substs: ['_#0r] }, OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: no-location (#0), ty: &S }, origin: TyAlias })])
  |
  = note: delayed at compiler/rustc_infer/src/infer/opaque_types/table.rs:50:26

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
   0:        0x104436658 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h502b9907255a9020
   1:        0x1044c1c58 - core::fmt::write::h1312fa7bb85e0d03
   2:        0x104467b54 - std::io::Write::write_fmt::hdc03145f6f1e9dcd
   3:        0x1044364ac - std::sys_common::backtrace::print::h3f2486f04aea89c9
   4:        0x10445fc70 - std::panicking::default_hook::{{closure}}::haa37fd6bee37c759
   5:        0x10445fa18 - std::panicking::default_hook::hc228b3d9e7765ab2
   6:        0x10cf9e394 - rustc_driver[6168a678ad3ae7e8]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x104460158 - std::panicking::rust_panic_with_hook::h780d45e68728c0ed
   8:        0x110fb31e4 - std[4c7b4fc7f8c44cca]::panicking::begin_panic::<rustc_errors[c285405409cce5f9]::ExplicitBug>::{closure#0}
   9:        0x110fb319c - std[4c7b4fc7f8c44cca]::sys_common::backtrace::__rust_end_short_backtrace::<std[4c7b4fc7f8c44cca]::panicking::begin_panic<rustc_errors[c285405409cce5f9]::ExplicitBug>::{closure#0}, !>
  10:        0x1112e7314 - std[4c7b4fc7f8c44cca]::panicking::begin_panic::<rustc_errors[c285405409cce5f9]::ExplicitBug>
  11:        0x110fbc4f0 - std[4c7b4fc7f8c44cca]::panic::panic_any::<rustc_errors[c285405409cce5f9]::ExplicitBug>
  12:        0x110fbeab4 - <rustc_errors[c285405409cce5f9]::HandlerInner>::flush_delayed::<alloc[8dbf09e20a862c28]::vec::Vec<rustc_errors[c285405409cce5f9]::diagnostic::Diagnostic>, &str>
  13:        0x110fab9f4 - <rustc_errors[c285405409cce5f9]::HandlerInner as core[e78181ecd1ceeb9c]::ops::drop::Drop>::drop
  14:        0x10cfe26ac - core[e78181ecd1ceeb9c]::ptr::drop_in_place::<rustc_errors[c285405409cce5f9]::Handler>
  15:        0x10cfe4a94 - core[e78181ecd1ceeb9c]::ptr::drop_in_place::<rustc_session[7b790e6398402969]::parse::ParseSess>
  16:        0x10cfca214 - core[e78181ecd1ceeb9c]::ptr::drop_in_place::<alloc[8dbf09e20a862c28]::rc::Rc<rustc_session[7b790e6398402969]::session::Session>>
  17:        0x10cfed760 - core[e78181ecd1ceeb9c]::ptr::drop_in_place::<rustc_interface[bd25737d3859b110]::interface::Compiler>
  18:        0x10cf8fc14 - rustc_span[dbe5a3cbc50f2bf]::with_source_map::<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_interface[bd25737d3859b110]::interface::create_compiler_and_run<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}>::{closure#1}>
  19:        0x10cf911ac - rustc_interface[bd25737d3859b110]::interface::create_compiler_and_run::<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}>
  20:        0x10cf7ed80 - <scoped_tls[e8ecb510089da108]::ScopedKey<rustc_span[dbe5a3cbc50f2bf]::SessionGlobals>>::set::<rustc_interface[bd25737d3859b110]::interface::run_compiler<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>
  21:        0x10cfacd6c - std[4c7b4fc7f8c44cca]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bd25737d3859b110]::util::run_in_thread_pool_with_globals<rustc_interface[bd25737d3859b110]::interface::run_compiler<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>
  22:        0x10cfa2b60 - std[4c7b4fc7f8c44cca]::panicking::try::<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, core[e78181ecd1ceeb9c]::panic::unwind_safe::AssertUnwindSafe<<std[4c7b4fc7f8c44cca]::thread::Builder>::spawn_unchecked_<rustc_interface[bd25737d3859b110]::util::run_in_thread_pool_with_globals<rustc_interface[bd25737d3859b110]::interface::run_compiler<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:        0x10cfa4060 - <<std[4c7b4fc7f8c44cca]::thread::Builder>::spawn_unchecked_<rustc_interface[bd25737d3859b110]::util::run_in_thread_pool_with_globals<rustc_interface[bd25737d3859b110]::interface::run_compiler<core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>, rustc_driver[6168a678ad3ae7e8]::run_compiler::{closure#1}>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>::{closure#0}, core[e78181ecd1ceeb9c]::result::Result<(), rustc_errors[c285405409cce5f9]::ErrorGuaranteed>>::{closure#1} as core[e78181ecd1ceeb9c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:        0x1044553a8 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3a51253c442e9fd2
  25:        0x10445cf80 - std::sys::unix::thread::Thread::new::thread_start::hca7e919e365a1d36
  26:        0x1b385c26c - __pthread_deallocate

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-dev running on aarch64-apple-darwin

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
[2]    43547 abort      ./miri run tabula.rs
