
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: unexpected const parent in type_of(): TypeBinding(TypeBinding { hir_id: HirId { owner: OwnerId { def_id: DefId(0:2 ~ foo[dca6]::T::A) }, local_id: 3 }, ident: X#0, gen_args: GenericArgs { args: [], bindings: [], parenthesized: false, span_ext: no-location (#0) }, kind: Equality { term: Const(AnonConst { hir_id: HirId { owner: OwnerId { def_id: DefId(0:2 ~ foo[dca6]::T::A) }, local_id: 1 }, body: BodyId { hir_id: HirId { owner: OwnerId { def_id: DefId(0:2 ~ foo[dca6]::T::A) }, local_id: 2 } } }) }, span: foo.rs:8:17: 8:25 (#0) })
  |
  = note: delayed at compiler/rustc_hir_analysis/src/collect/type_of.rs:456:23

error: internal compiler error: Const::from_anon_const: couldn't lit_to_const TypeError
 --> foo.rs:8:21
  |
8 |     type A: S<C<X = 0i32> = 34>;
  |                     ^^^^
  |
  = note: delayed at compiler/rustc_middle/src/ty/consts.rs:122:30

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1542:13
stack backtrace:
   0:     0x7fdf469a2090 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h69c219382a706de4
   1:     0x7fdf46a0f84e - core::fmt::write::hfaba6c7f5d324983
   2:     0x7fdf469716d5 - std::io::Write::write_fmt::he0f5763098f70719
   3:     0x7fdf46984e04 - std::panicking::default_hook::{{closure}}::hadd9f7be4096b7a1
   4:     0x7fdf46984a58 - std::panicking::default_hook::h7c40de63bd1ac842
   5:     0x7fdf4710b826 - rustc_driver[bf5b15a4efa2da32]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fdf469855bd - std::panicking::rust_panic_with_hook::hd33806f1a523247b
   7:     0x7fdf49a967a3 - std[77418543e64ae431]::panicking::begin_panic::<rustc_errors[e621a6510cc517c5]::ExplicitBug>::{closure#0}
   8:     0x7fdf49a95cd6 - std[77418543e64ae431]::sys_common::backtrace::__rust_end_short_backtrace::<std[77418543e64ae431]::panicking::begin_panic<rustc_errors[e621a6510cc517c5]::ExplicitBug>::{closure#0}, !>
   9:     0x7fdf470e5496 - std[77418543e64ae431]::panicking::begin_panic::<rustc_errors[e621a6510cc517c5]::ExplicitBug>
  10:     0x7fdf49a8b096 - std[77418543e64ae431]::panic::panic_any::<rustc_errors[e621a6510cc517c5]::ExplicitBug>
  11:     0x7fdf49a93579 - <rustc_errors[e621a6510cc517c5]::HandlerInner>::flush_delayed::<alloc[344271dbb998174e]::vec::Vec<rustc_errors[e621a6510cc517c5]::diagnostic::Diagnostic>, &str>
  12:     0x7fdf49a91b6a - <rustc_errors[e621a6510cc517c5]::Handler>::abort_if_errors_or_delayed_span_bugs
  13:     0x55bf9d4f5f5e - rustdoc[3755b0600f342e0c]::clean::clean_generic_args
  14:     0x55bf9d4f5e38 - rustdoc[3755b0600f342e0c]::clean::clean_generic_args
  15:     0x55bf9d5f381d - <alloc[344271dbb998174e]::vec::Vec<rustdoc[3755b0600f342e0c]::clean::types::PathSegment> as alloc[344271dbb998174e]::vec::spec_from_iter::SpecFromIter<rustdoc[3755b0600f342e0c]::clean::types::PathSegment, core[d55ced6309a25c5f]::iter::adapters::map::Map<core[d55ced6309a25c5f]::slice::iter::Iter<rustc_hir[854dfe8484ba389]::hir::PathSegment>, rustdoc[3755b0600f342e0c]::clean::clean_path::{closure#0}>>>::from_iter
  16:     0x55bf9d4ef83e - rustdoc[3755b0600f342e0c]::clean::clean_poly_trait_ref
  17:     0x55bf9d4eaa0b - rustdoc[3755b0600f342e0c]::clean::clean_generic_bound
  18:     0x55bf9d5f46ae - <alloc[344271dbb998174e]::vec::Vec<rustdoc[3755b0600f342e0c]::clean::types::GenericBound> as alloc[344271dbb998174e]::vec::spec_from_iter::SpecFromIter<rustdoc[3755b0600f342e0c]::clean::types::GenericBound, core[d55ced6309a25c5f]::iter::adapters::filter_map::FilterMap<core[d55ced6309a25c5f]::slice::iter::Iter<rustc_hir[854dfe8484ba389]::hir::GenericBound>, rustdoc[3755b0600f342e0c]::clean::clean_maybe_renamed_item::{closure#1}::{closure#0}>>>::from_iter
  19:     0x55bf9d3f6364 - <rustdoc[3755b0600f342e0c]::core::DocContext>::with_param_env::<rustdoc[3755b0600f342e0c]::clean::types::Item, rustdoc[3755b0600f342e0c]::clean::clean_trait_item::{closure#0}>
  20:     0x55bf9d5f9811 - <alloc[344271dbb998174e]::vec::Vec<rustdoc[3755b0600f342e0c]::clean::types::Item> as alloc[344271dbb998174e]::vec::spec_from_iter::SpecFromIter<rustdoc[3755b0600f342e0c]::clean::types::Item, core[d55ced6309a25c5f]::iter::adapters::map::Map<core[d55ced6309a25c5f]::slice::iter::Iter<rustc_hir[854dfe8484ba389]::hir::TraitItemRef>, rustdoc[3755b0600f342e0c]::clean::clean_maybe_renamed_item::{closure#1}::{closure#5}>>>::from_iter
  21:     0x55bf9d3f4197 - <rustdoc[3755b0600f342e0c]::core::DocContext>::with_param_env::<alloc[344271dbb998174e]::vec::Vec<rustdoc[3755b0600f342e0c]::clean::types::Item>, rustdoc[3755b0600f342e0c]::clean::clean_maybe_renamed_item::{closure#1}>
  22:     0x55bf9d4e8ffb - <&mut rustdoc[3755b0600f342e0c]::clean::clean_doc_module::{closure#2} as core[d55ced6309a25c5f]::ops::function::FnOnce<(&(&rustc_hir[854dfe8484ba389]::hir::Item, core[d55ced6309a25c5f]::option::Option<rustc_span[a05d955188b2abc7]::symbol::Symbol>),)>>::call_once
  23:     0x55bf9d5ea13a - <alloc[344271dbb998174e]::vec::Vec<rustdoc[3755b0600f342e0c]::clean::types::Item> as alloc[344271dbb998174e]::vec::spec_extend::SpecExtend<rustdoc[3755b0600f342e0c]::clean::types::Item, core[d55ced6309a25c5f]::iter::adapters::flatten::FlatMap<core[d55ced6309a25c5f]::slice::iter::Iter<(&rustc_hir[854dfe8484ba389]::hir::Item, core[d55ced6309a25c5f]::option::Option<rustc_span[a05d955188b2abc7]::symbol::Symbol>)>, alloc[344271dbb998174e]::vec::Vec<rustdoc[3755b0600f342e0c]::clean::types::Item>, rustdoc[3755b0600f342e0c]::clean::clean_doc_module::{closure#2}>>>::spec_extend
  24:     0x55bf9d4ea640 - rustdoc[3755b0600f342e0c]::clean::clean_doc_module
  25:     0x55bf9d679374 - rustdoc[3755b0600f342e0c]::clean::utils::krate
  26:     0x55bf9d50ce44 - <rustc_session[605dacab06639d89]::session::Session>::time::<rustdoc[3755b0600f342e0c]::clean::types::Crate, rustdoc[3755b0600f342e0c]::core::run_global_ctxt::{closure#4}>
  27:     0x55bf9d3fb9c1 - rustdoc[3755b0600f342e0c]::core::run_global_ctxt
  28:     0x55bf9d50d26f - <rustc_session[605dacab06639d89]::session::Session>::time::<(rustdoc[3755b0600f342e0c]::clean::types::Crate, rustdoc[3755b0600f342e0c]::config::RenderOptions, rustdoc[3755b0600f342e0c]::formats::cache::Cache), rustdoc[3755b0600f342e0c]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  29:     0x55bf9d463261 - <rustc_interface[158590035fe8886d]::passes::QueryContext>::enter::<rustdoc[3755b0600f342e0c]::main_options::{closure#0}::{closure#0}::{closure#1}, core[d55ced6309a25c5f]::result::Result<(), rustc_errors[e621a6510cc517c5]::ErrorGuaranteed>>
  30:     0x55bf9d3b536b - <rustc_interface[158590035fe8886d]::interface::Compiler>::enter::<rustdoc[3755b0600f342e0c]::main_options::{closure#0}::{closure#0}, core[d55ced6309a25c5f]::result::Result<(), rustc_errors[e621a6510cc517c5]::ErrorGuaranteed>>
  31:     0x55bf9d5834b0 - rustc_span[a05d955188b2abc7]::with_source_map::<core[d55ced6309a25c5f]::result::Result<(), rustc_errors[e621a6510cc517c5]::ErrorGuaranteed>, rustc_interface[158590035fe8886d]::interface::create_compiler_and_run<core[d55ced6309a25c5f]::result::Result<(), rustc_errors[e621a6510cc517c5]::ErrorGuaranteed>, rustdoc[3755b0600f342e0c]::main_options::{closure#0}>::{closure#1}>
  32:     0x55bf9d3b949e - rustc_interface[158590035fe8886d]::interface::create_compiler_and_run::<core[d55ced6309a25c5f]::result::Result<(), rustc_errors[e621a6510cc517c5]::ErrorGuaranteed>, rustdoc[3755b0600f342e0c]::main_options::{closure#0}>
  33:     0x55bf9d3b36e0 - rustdoc[3755b0600f342e0c]::main_options
  34:     0x55bf9d3b693b - <scoped_tls[10d8a9c768e14d1c]::ScopedKey<rustc_span[a05d955188b2abc7]::SessionGlobals>>::set::<rustdoc[3755b0600f342e0c]::main_args::{closure#0}, core[d55ced6309a25c5f]::result::Result<(), rustc_errors[e621a6510cc517c5]::ErrorGuaranteed>>
  35:     0x55bf9d522de0 - std[77418543e64ae431]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[158590035fe8886d]::util::run_in_thread_pool_with_globals<rustdoc[3755b0600f342e0c]::main_args::{closure#0}, core[d55ced6309a25c5f]::result::Result<(), rustc_errors[e621a6510cc517c5]::ErrorGuaranteed>>::{closure#0}, core[d55ced6309a25c5f]::result::Result<(), rustc_errors[e621a6510cc517c5]::ErrorGuaranteed>>
  36:     0x55bf9d3e80ac - <<std[77418543e64ae431]::thread::Builder>::spawn_unchecked_<rustc_interface[158590035fe8886d]::util::run_in_thread_pool_with_globals<rustdoc[3755b0600f342e0c]::main_args::{closure#0}, core[d55ced6309a25c5f]::result::Result<(), rustc_errors[e621a6510cc517c5]::ErrorGuaranteed>>::{closure#0}, core[d55ced6309a25c5f]::result::Result<(), rustc_errors[e621a6510cc517c5]::ErrorGuaranteed>>::{closure#1} as core[d55ced6309a25c5f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7fdf4695eaf3 - std::sys::unix::thread::Thread::new::thread_start::heafa918dcc1208d8
  38:     0x7fdf46624b43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  39:     0x7fdf466b6a00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  40:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
error: aborting due to 3 previous errors
