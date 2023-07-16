
error[E0107]: missing generics for trait `A`
 --> src/lib.rs:3:24
  |
3 | struct B<C: ?Sized, D: A> {}
  |                        ^ expected 1 generic argument
  |
note: trait defined here, with 1 generic parameter: `T`
 --> src/lib.rs:1:7
  |
1 | trait A<T> {}
  |       ^ -
help: add missing generic argument
  |
3 | struct B<C: ?Sized, D: A<T>> {}
  |                        ~~~~

thread 'rustc' panicked at 'Error', src/librustdoc/clean/mod.rs:1737:25
stack backtrace:
   0:     0x7ffa1bbead50 - std::backtrace_rs::backtrace::libunwind::trace::h22157d54c25d05ad
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7ffa1bbead50 - std::backtrace_rs::backtrace::trace_unsynchronized::hf9a98b3986ed5962
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7ffa1bbead50 - std::sys_common::backtrace::_print_fmt::h44f8a801ddb575b4
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7ffa1bbead50 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h306abe8d5f5fb26d
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7ffa1bc45aae - core::fmt::write::h5a3c14feb177cc6f
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/core/src/fmt/mod.rs:1202:17
   5:     0x7ffa1bbdb865 - std::io::Write::write_fmt::h2fac6c752c3fce7e
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/io/mod.rs:1679:15
   6:     0x7ffa1bbeda03 - std::sys_common::backtrace::_print::h17490259a7578aec
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7ffa1bbeda03 - std::sys_common::backtrace::print::h6dba4ce5923cf00b
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7ffa1bbeda03 - std::panicking::default_hook::{{closure}}::h25daedc6802fa9d1
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/panicking.rs:295:22
   9:     0x7ffa1bbed6ef - std::panicking::default_hook::hffec061dc0fa1a5a
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/panicking.rs:314:9
  10:     0x7ffa1e3fc951 - <rustc_driver[2817d6214121e090]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[7a17016bf310e64d]::ops::function::FnOnce<(&core[7a17016bf310e64d]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7ffa1bbee23d - std::panicking::rust_panic_with_hook::h74e697c31d32505c
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/panicking.rs:702:17
  12:     0x7ffa1bbee051 - std::panicking::begin_panic_handler::{{closure}}::hd0b9cdbd524ed365
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/panicking.rs:586:13
  13:     0x7ffa1bbeb1fc - std::sys_common::backtrace::__rust_end_short_backtrace::h3feb3f4e0604d329
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7ffa1bbeddb2 - rust_begin_unwind
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/panicking.rs:584:5
  15:     0x7ffa1bc42683 - core::panicking::panic_fmt::h0f41ee9e7f6526dd
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/core/src/panicking.rs:142:14
  16:     0x55def3e243ac - rustdoc[a4cb4ff8fe5b9369]::clean::clean_middle_ty
  17:     0x55def3c0268d - <&mut rustdoc[a4cb4ff8fe5b9369]::clean::utils::substs_to_args::{closure#0} as core[7a17016bf310e64d]::ops::function::FnMut<(&rustc_middle[e9ef365c3785ee8]::ty::subst::GenericArg,)>>::call_mut
  18:     0x55def3c4665e - <alloc[e7766d434969df5]::vec::Vec<rustdoc[a4cb4ff8fe5b9369]::clean::types::GenericArg> as alloc[e7766d434969df5]::vec::spec_extend::SpecExtend<rustdoc[a4cb4ff8fe5b9369]::clean::types::GenericArg, core[7a17016bf310e64d]::iter::adapters::filter_map::FilterMap<core[7a17016bf310e64d]::slice::iter::Iter<rustc_middle[e9ef365c3785ee8]::ty::subst::GenericArg>, rustdoc[a4cb4ff8fe5b9369]::clean::utils::substs_to_args::{closure#0}>>>::spec_extend
  19:     0x55def3c0b4f8 - rustdoc[a4cb4ff8fe5b9369]::clean::utils::external_path
  20:     0x55def3e19c09 - rustdoc[a4cb4ff8fe5b9369]::clean::clean_trait_ref_with_bindings
  21:     0x55def3e1a0de - rustdoc[a4cb4ff8fe5b9369]::clean::clean_poly_trait_ref_with_bindings
  22:     0x55def3e1aa93 - rustdoc[a4cb4ff8fe5b9369]::clean::clean_predicate
  23:     0x55def3c53fcf - <alloc[e7766d434969df5]::vec::Vec<rustdoc[a4cb4ff8fe5b9369]::clean::types::WherePredicate> as alloc[e7766d434969df5]::vec::spec_from_iter::SpecFromIter<rustdoc[a4cb4ff8fe5b9369]::clean::types::WherePredicate, core[7a17016bf310e64d]::iter::adapters::flatten::FlatMap<alloc[e7766d434969df5]::vec::into_iter::IntoIter<&rustc_middle[e9ef365c3785ee8]::ty::Predicate>, core[7a17016bf310e64d]::option::Option<rustdoc[a4cb4ff8fe5b9369]::clean::types::WherePredicate>, rustdoc[a4cb4ff8fe5b9369]::clean::clean_ty_generics::{closure#3}>>>::from_iter
  24:     0x55def3e1d041 - rustdoc[a4cb4ff8fe5b9369]::clean::clean_ty_generics
  25:     0x55def3bd140e - <rustdoc[a4cb4ff8fe5b9369]::clean::auto_trait::AutoTraitFinder>::param_env_to_generics
  26:     0x55def3c80067 - <rustc_trait_selection[df340bb754643847]::traits::auto_trait::AutoTraitFinder>::find_auto_trait_generics::<rustdoc[a4cb4ff8fe5b9369]::clean::types::Generics, <rustdoc[a4cb4ff8fe5b9369]::clean::auto_trait::AutoTraitFinder>::generate_for_trait::{closure#0}>
  27:     0x55def3bce2a6 - <rustdoc[a4cb4ff8fe5b9369]::clean::auto_trait::AutoTraitFinder>::generate_for_trait
  28:     0x55def3c67b51 - <alloc[e7766d434969df5]::vec::Vec<rustdoc[a4cb4ff8fe5b9369]::clean::types::Item> as alloc[e7766d434969df5]::vec::spec_from_iter::SpecFromIter<rustdoc[a4cb4ff8fe5b9369]::clean::types::Item, core[7a17016bf310e64d]::iter::adapters::filter_map::FilterMap<alloc[e7766d434969df5]::vec::into_iter::IntoIter<rustc_span[8a07e301efc44469]::def_id::DefId>, <rustdoc[a4cb4ff8fe5b9369]::clean::auto_trait::AutoTraitFinder>::get_auto_trait_impls::{closure#0}>>>::from_iter
  29:     0x55def3bcf002 - <rustdoc[a4cb4ff8fe5b9369]::clean::auto_trait::AutoTraitFinder>::get_auto_trait_impls
  30:     0x55def3c0ddd1 - rustdoc[a4cb4ff8fe5b9369]::clean::utils::get_auto_trait_and_blanket_impls
  31:     0x55def3dc1774 - <rustdoc[a4cb4ff8fe5b9369]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[a4cb4ff8fe5b9369]::visit::DocVisitor>::visit_item
  32:     0x55def3dc18ba - <rustdoc[a4cb4ff8fe5b9369]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[a4cb4ff8fe5b9369]::visit::DocVisitor>::visit_item
  33:     0x55def3dc27ed - <rustdoc[a4cb4ff8fe5b9369]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[a4cb4ff8fe5b9369]::visit::DocVisitor>::visit_crate
  34:     0x55def3e3b6e2 - <rustc_session[920c8e110a6e000f]::session::Session>::time::<alloc[e7766d434969df5]::vec::Vec<rustdoc[a4cb4ff8fe5b9369]::clean::types::Item>, rustdoc[a4cb4ff8fe5b9369]::passes::collect_trait_impls::collect_trait_impls::{closure#0}>
  35:     0x55def3dbf602 - rustdoc[a4cb4ff8fe5b9369]::passes::collect_trait_impls::collect_trait_impls
  36:     0x55def3e3bcbd - <rustc_session[920c8e110a6e000f]::session::Session>::time::<rustdoc[a4cb4ff8fe5b9369]::clean::types::Crate, rustdoc[a4cb4ff8fe5b9369]::core::run_global_ctxt::{closure#7}>
  37:     0x55def3d144b8 - rustdoc[a4cb4ff8fe5b9369]::core::run_global_ctxt
  38:     0x55def3e3bf9f - <rustc_session[920c8e110a6e000f]::session::Session>::time::<(rustdoc[a4cb4ff8fe5b9369]::clean::types::Crate, rustdoc[a4cb4ff8fe5b9369]::config::RenderOptions, rustdoc[a4cb4ff8fe5b9369]::formats::cache::Cache), rustdoc[a4cb4ff8fe5b9369]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  39:     0x55def3d86531 - <rustc_interface[2d255db050a36d31]::passes::QueryContext>::enter::<rustdoc[a4cb4ff8fe5b9369]::main_options::{closure#0}::{closure#0}::{closure#1}, core[7a17016bf310e64d]::result::Result<(), rustc_errors[aa542ed4628157d3]::ErrorGuaranteed>>
  40:     0x55def3b6c8ea - <rustc_interface[2d255db050a36d31]::interface::Compiler>::enter::<rustdoc[a4cb4ff8fe5b9369]::main_options::{closure#0}::{closure#0}, core[7a17016bf310e64d]::result::Result<(), rustc_errors[aa542ed4628157d3]::ErrorGuaranteed>>
  41:     0x55def3eaf450 - rustc_span[8a07e301efc44469]::with_source_map::<core[7a17016bf310e64d]::result::Result<(), rustc_errors[aa542ed4628157d3]::ErrorGuaranteed>, rustc_interface[2d255db050a36d31]::interface::create_compiler_and_run<core[7a17016bf310e64d]::result::Result<(), rustc_errors[aa542ed4628157d3]::ErrorGuaranteed>, rustdoc[a4cb4ff8fe5b9369]::main_options::{closure#0}>::{closure#1}>
  42:     0x55def3baf965 - rustdoc[a4cb4ff8fe5b9369]::main_options
  43:     0x55def3d842cb - <scoped_tls[f167104904068c68]::ScopedKey<rustc_span[8a07e301efc44469]::SessionGlobals>>::set::<rustdoc[a4cb4ff8fe5b9369]::main_args::{closure#0}, core[7a17016bf310e64d]::result::Result<(), rustc_errors[aa542ed4628157d3]::ErrorGuaranteed>>
  44:     0x55def3e518c0 - std[7414053366649774]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2d255db050a36d31]::util::run_in_thread_pool_with_globals<rustdoc[a4cb4ff8fe5b9369]::main_args::{closure#0}, core[7a17016bf310e64d]::result::Result<(), rustc_errors[aa542ed4628157d3]::ErrorGuaranteed>>::{closure#0}, core[7a17016bf310e64d]::result::Result<(), rustc_errors[aa542ed4628157d3]::ErrorGuaranteed>>
  45:     0x55def3d01b5c - <<std[7414053366649774]::thread::Builder>::spawn_unchecked_<rustc_interface[2d255db050a36d31]::util::run_in_thread_pool_with_globals<rustdoc[a4cb4ff8fe5b9369]::main_args::{closure#0}, core[7a17016bf310e64d]::result::Result<(), rustc_errors[aa542ed4628157d3]::ErrorGuaranteed>>::{closure#0}, core[7a17016bf310e64d]::result::Result<(), rustc_errors[aa542ed4628157d3]::ErrorGuaranteed>>::{closure#1} as core[7a17016bf310e64d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  46:     0x7ffa1bbf8013 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h59c19dbfda82d3be
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/alloc/src/boxed.rs:1940:9
  47:     0x7ffa1bbf8013 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h5cc1cabf17fe5922
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/alloc/src/boxed.rs:1940:9
  48:     0x7ffa1bbf8013 - std::sys::unix::thread::Thread::new::thread_start::h8f3eeb92c9304ef6
                               at /rustc/432abd86f231c908f6df3cdd779e83f35084be90/library/std/src/sys/unix/thread.rs:108:17
  49:     0x7ffa1b95974d - <unknown>
  50:     0x7ffa1b9db700 - <unknown>
  51:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (432abd86f 2022-09-20) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.
