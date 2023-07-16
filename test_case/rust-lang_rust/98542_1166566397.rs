plain
---- [rustdoc] src/test/rustdoc/const-generics/generic_const_exprs.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/generic_const_exprs/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/generic_const_exprs" "--deny" "warnings" "/checkout/src/test/rustdoc/const-generics/generic_const_exprs.rs"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'not user writable', src/librustdoc/clean/mod.rs:303:63
   0:     0x7f5e67d9ba6c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4d0d61aea20f239b
   1:     0x7f5e67e02928 - core::fmt::write::hfc0402a2eea8e601
   1:     0x7f5e67e02928 - core::fmt::write::hfc0402a2eea8e601
   2:     0x7f5e67d8b871 - std::io::Write::write_fmt::hc3382af6bfd17e3a
   3:     0x7f5e67d9ea5e - std::panicking::default_hook::{{closure}}::h704dcb7dd91b3a17
   4:     0x7f5e67d9e749 - std::panicking::default_hook::hfc566923e5ebb514
   5:     0x7f5e688bc484 - rustc_driver[790dd00f8c97ce97]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5e67d9f1c2 - std::panicking::rust_panic_with_hook::hf72bcc1b9732a821
   7:     0x7f5e67d9efb9 - std::panicking::begin_panic_handler::{{closure}}::h916fcc345962d16a
   8:     0x7f5e67d9bf84 - std::sys_common::backtrace::__rust_end_short_backtrace::h0eb9b19131732088
   9:     0x7f5e67d9ecd9 - rust_begin_unwind
  10:     0x7f5e67d531c3 - core::panicking::panic_fmt::hf27594c85b344e2a
  11:     0x55582a69be8f - <rustc_middle[4ceeac13f465b725]::ty::Predicate as rustdoc[a029b3f8aa973f77]::clean::Clean<core[20488d6b8f09a48e]::option::Option<rustdoc[a029b3f8aa973f77]::clean::types::WherePredicate>>>::clean
  12:     0x55582a9762c0 - <alloc[c79065ebc8b87e02]::vec::Vec<rustdoc[a029b3f8aa973f77]::clean::types::WherePredicate> as alloc[c79065ebc8b87e02]::vec::spec_from_iter::SpecFromIter<rustdoc[a029b3f8aa973f77]::clean::types::WherePredicate, core[20488d6b8f09a48e]::iter::adapters::flatten::FlatMap<alloc[c79065ebc8b87e02]::vec::into_iter::IntoIter<&rustc_middle[4ceeac13f465b725]::ty::Predicate>, core[20488d6b8f09a48e]::option::Option<rustdoc[a029b3f8aa973f77]::clean::types::WherePredicate>, rustdoc[a029b3f8aa973f77]::clean::clean_ty_generics::{closure#3}>>>::from_iter
  13:     0x55582a8d0b58 - rustdoc[a029b3f8aa973f77]::clean::clean_ty_generics
  14:     0x55582a8287c0 - <rustdoc[a029b3f8aa973f77]::clean::auto_trait::AutoTraitFinder>::param_env_to_generics
  15:     0x55582a825f22 - <rustdoc[a029b3f8aa973f77]::clean::auto_trait::AutoTraitFinder>::generate_for_trait
  16:     0x55582a819679 - <&mut <rustdoc[a029b3f8aa973f77]::clean::auto_trait::AutoTraitFinder>::get_auto_trait_impls::{closure#0} as core[20488d6b8f09a48e]::ops::function::FnMut<(rustc_span[a95baa0df7e238dd]::def_id::DefId,)>>::call_mut
  17:     0x55582a98c18c - <alloc[c79065ebc8b87e02]::vec::Vec<rustdoc[a029b3f8aa973f77]::clean::types::Item> as alloc[c79065ebc8b87e02]::vec::spec_from_iter::SpecFromIter<rustdoc[a029b3f8aa973f77]::clean::types::Item, core[20488d6b8f09a48e]::iter::adapters::filter_map::FilterMap<alloc[c79065ebc8b87e02]::vec::into_iter::IntoIter<rustc_span[a95baa0df7e238dd]::def_id::DefId>, <rustdoc[a029b3f8aa973f77]::clean::auto_trait::AutoTraitFinder>::get_auto_trait_impls::{closure#0}>>>::from_iter
  18:     0x55582a827c51 - <rustdoc[a029b3f8aa973f77]::clean::auto_trait::AutoTraitFinder>::get_auto_trait_impls
  19:     0x55582a79084b - rustdoc[a029b3f8aa973f77]::clean::utils::get_auto_trait_and_blanket_impls
  20:     0x55582a85bc64 - <rustdoc[a029b3f8aa973f77]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[a029b3f8aa973f77]::visit::DocVisitor>::visit_item
  21:     0x55582a861c1d - <rustdoc[a029b3f8aa973f77]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[a029b3f8aa973f77]::visit::DocVisitor>::visit_inner_recur
  22:     0x55582a85bcb8 - <rustdoc[a029b3f8aa973f77]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[a029b3f8aa973f77]::visit::DocVisitor>::visit_item
  23:     0x55582a862bfd - <rustdoc[a029b3f8aa973f77]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[a029b3f8aa973f77]::visit::DocVisitor>::visit_crate
  24:     0x55582a866995 - <rustc_session[d67843941c0d49d]::session::Session>::time::<alloc[c79065ebc8b87e02]::vec::Vec<rustdoc[a029b3f8aa973f77]::clean::types::Item>, rustdoc[a029b3f8aa973f77]::passes::collect_trait_impls::collect_trait_impls::{closure#0}>
  25:     0x55582a85a1a2 - rustdoc[a029b3f8aa973f77]::passes::collect_trait_impls::collect_trait_impls
  26:     0x55582a866ba0 - <rustc_session[d67843941c0d49d]::session::Session>::time::<rustdoc[a029b3f8aa973f77]::clean::types::Crate, rustdoc[a029b3f8aa973f77]::core::run_global_ctxt::{closure#8}>
  27:     0x55582a836beb - rustdoc[a029b3f8aa973f77]::core::run_global_ctxt
  28:     0x55582a866e1e - <rustc_session[d67843941c0d49d]::session::Session>::time::<(rustdoc[a029b3f8aa973f77]::clean::types::Crate, rustdoc[a029b3f8aa973f77]::config::RenderOptions, rustdoc[a029b3f8aa973f77]::formats::cache::Cache), rustdoc[a029b3f8aa973f77]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  29:     0x55582a7fe4bc - <rustc_interface[96840b0acd7a3f5f]::passes::QueryContext>::enter::<rustdoc[a029b3f8aa973f77]::main_options::{closure#0}::{closure#0}::{closure#1}, core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>>
  30:     0x55582a75bff5 - <rustc_interface[96840b0acd7a3f5f]::interface::Compiler>::enter::<rustdoc[a029b3f8aa973f77]::main_options::{closure#0}::{closure#0}, core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>>
  31:     0x55582a639656 - rustc_span[a95baa0df7e238dd]::with_source_map::<core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>, rustc_interface[96840b0acd7a3f5f]::interface::create_compiler_and_run<core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>, rustdoc[a029b3f8aa973f77]::main_options::{closure#0}>::{closure#1}>
  32:     0x55582a77802b - rustc_interface[96840b0acd7a3f5f]::interface::create_compiler_and_run::<core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>, rustdoc[a029b3f8aa973f77]::main_options::{closure#0}>
  33:     0x55582a6739b0 - rustdoc[a029b3f8aa973f77]::main_options
  34:     0x55582a8e6eac - <scoped_tls[9cf97bbb7ae2969a]::ScopedKey<rustc_span[a95baa0df7e238dd]::SessionGlobals>>::set::<rustdoc[a029b3f8aa973f77]::main_args::{closure#0}, core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>>
  35:     0x55582a77876f - std[85198306a4c527cc]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[96840b0acd7a3f5f]::util::run_in_thread_pool_with_globals<rustdoc[a029b3f8aa973f77]::main_args::{closure#0}, core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>>::{closure#0}, core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>>
  36:     0x55582a647cd1 - std[85198306a4c527cc]::panicking::try::<core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>, core[20488d6b8f09a48e]::panic::unwind_safe::AssertUnwindSafe<<std[85198306a4c527cc]::thread::Builder>::spawn_unchecked_<rustc_interface[96840b0acd7a3f5f]::util::run_in_thread_pool_with_globals<rustdoc[a029b3f8aa973f77]::main_args::{closure#0}, core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>>::{closure#0}, core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  37:     0x55582a816c72 - <<std[85198306a4c527cc]::thread::Builder>::spawn_unchecked_<rustc_interface[96840b0acd7a3f5f]::util::run_in_thread_pool_with_globals<rustdoc[a029b3f8aa973f77]::main_args::{closure#0}, core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>>::{closure#0}, core[20488d6b8f09a48e]::result::Result<(), rustc_errors[8521caabe9693b6e]::ErrorGuaranteed>>::{closure#1} as core[20488d6b8f09a48e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f5e67daa5a3 - std::sys::unix::thread::Thread::new::thread_start::h56c135c8b85581fc
  39:     0x7f5e67cad609 - start_thread
  40:     0x7f5e67a81133 - clone
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

