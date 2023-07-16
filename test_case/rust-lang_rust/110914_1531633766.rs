plain
........................................................................................ 264/616
........................................................................................ 352/616
.....................................................................i.................. 440/616
..........................................................................i............. 528/616
......................F........F........................................................
failures:

---- [rustdoc] tests/rustdoc/synthetic_auto/lifetimes.rs stdout ----


error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/lifetimes/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/lifetimes" "--deny" "warnings" "/checkout/tests/rustdoc/synthetic_auto/lifetimes.rs"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Unexpected result when selecting Foo<'c, K> Obligation(predicate=Binder(ProjectionPredicate(AliasTy { substs: [K, (&'b bool,)], def_id: DefId(2:2920 ~ core[0cff]::ops::function::FnOnce::Output) }, Term::Ty(&u8)), [Region(BrNamed(DefId(0:10 ~ lifetimes[a36f]::{impl#0}::'b), 'b))]), depth=1)', compiler/rustc_trait_selection/src/traits/auto_trait.rs:756:33
stack backtrace:
   0:     0x7f470528f4b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8f3a36524dc33c10
   1:     0x7f47052f68c8 - core::fmt::write::h2dcfe570188983db
   2:     0x7f4705283b21 - std::io::Write::write_fmt::hf19c091b65ccf871
   2:     0x7f4705283b21 - std::io::Write::write_fmt::hf19c091b65ccf871
   3:     0x7f470528f2c1 - std::sys_common::backtrace::print::h959a920b6fb86385
   4:     0x7f470529244a - std::panicking::default_hook::{{closure}}::h9f373ba5b884b663
   5:     0x7f470529212c - std::panicking::default_hook::ha043523a77230732
   6:     0x7f4705d54265 - rustc_driver_impl[3ef7316c9936ed4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f4705292b57 - std::panicking::rust_panic_with_hook::hd13c922f40bf5cf4
   8:     0x7f47052928d7 - std::panicking::begin_panic_handler::{{closure}}::h2e927f37f4fe117e
   9:     0x7f470528f996 - std::sys_common::backtrace::__rust_end_short_backtrace::h6a48c2e8175db2c9
  10:     0x7f47052925c7 - rust_begin_unwind
  11:     0x7f47052470c3 - core::panicking::panic_fmt::h6573ff4054080c8e
  12:     0x7f47086ff8ad - <rustc_trait_selection[255c1400b4081eec]::traits::auto_trait::AutoTraitFinder>::evaluate_predicates
  13:     0x55933da1e580 - <rustc_trait_selection[255c1400b4081eec]::traits::auto_trait::AutoTraitFinder>::find_auto_trait_generics::<rustdoc[97eab9d96eb2a784]::clean::types::Generics, <rustdoc[97eab9d96eb2a784]::clean::auto_trait::AutoTraitFinder>::generate_for_trait::{closure#0}>
  14:     0x55933d96795e - <rustdoc[97eab9d96eb2a784]::clean::auto_trait::AutoTraitFinder>::generate_for_trait
  15:     0x55933d95e169 - <&mut <rustdoc[97eab9d96eb2a784]::clean::auto_trait::AutoTraitFinder>::get_auto_trait_impls::{closure#0} as core[cff1766242b3081]::ops::function::FnMut<(rustc_span[46ba486e373c9e8b]::def_id::DefId,)>>::call_mut
  16:     0x55933dcb3e73 - <alloc[3d170235524188ba]::vec::Vec<rustdoc[97eab9d96eb2a784]::clean::types::Item> as alloc[3d170235524188ba]::vec::spec_from_iter::SpecFromIter<rustdoc[97eab9d96eb2a784]::clean::types::Item, core[cff1766242b3081]::iter::adapters::filter_map::FilterMap<alloc[3d170235524188ba]::vec::into_iter::IntoIter<rustc_span[46ba486e373c9e8b]::def_id::DefId>, <rustdoc[97eab9d96eb2a784]::clean::auto_trait::AutoTraitFinder>::get_auto_trait_impls::{closure#0}>>>::from_iter
  17:     0x55933d976ce1 - rustdoc[97eab9d96eb2a784]::clean::utils::get_auto_trait_and_blanket_impls
  18:     0x55933dbf7e8a - <rustdoc[97eab9d96eb2a784]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[97eab9d96eb2a784]::visit::DocVisitor>::visit_item
  19:     0x55933dbf804a - <rustdoc[97eab9d96eb2a784]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[97eab9d96eb2a784]::visit::DocVisitor>::visit_item
  20:     0x55933dbf8e61 - <rustdoc[97eab9d96eb2a784]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[97eab9d96eb2a784]::visit::DocVisitor>::visit_crate
  21:     0x55933d99e408 - <rustc_session[1b4ec8bcd408194c]::session::Session>::time::<alloc[3d170235524188ba]::vec::Vec<rustdoc[97eab9d96eb2a784]::clean::types::Item>, rustdoc[97eab9d96eb2a784]::passes::collect_trait_impls::collect_trait_impls::{closure#0}>
  22:     0x55933dbf674a - rustdoc[97eab9d96eb2a784]::passes::collect_trait_impls::collect_trait_impls
  23:     0x55933d99e651 - <rustc_session[1b4ec8bcd408194c]::session::Session>::time::<rustdoc[97eab9d96eb2a784]::clean::types::Crate, rustdoc[97eab9d96eb2a784]::core::run_global_ctxt::{closure#8}>
  24:     0x55933daad8b3 - rustdoc[97eab9d96eb2a784]::core::run_global_ctxt
  25:     0x55933d99e8b3 - <rustc_session[1b4ec8bcd408194c]::session::Session>::time::<(rustdoc[97eab9d96eb2a784]::clean::types::Crate, rustdoc[97eab9d96eb2a784]::config::RenderOptions, rustdoc[97eab9d96eb2a784]::formats::cache::Cache), rustdoc[97eab9d96eb2a784]::main_args::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
  26:     0x55933dab6b0b - <rustc_middle[ffbba00c621c13f]::ty::context::GlobalCtxt>::enter::<rustdoc[97eab9d96eb2a784]::main_args::{closure#1}::{closure#0}::{closure#0}, core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>>
  27:     0x55933da1d65c - <rustc_interface[90a55842af0da270]::queries::QueryResult<&rustc_middle[ffbba00c621c13f]::ty::context::GlobalCtxt>>::enter::<core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>, rustdoc[97eab9d96eb2a784]::main_args::{closure#1}::{closure#0}::{closure#0}>
  28:     0x55933db886cd - <rustc_interface[90a55842af0da270]::interface::Compiler>::enter::<rustdoc[97eab9d96eb2a784]::main_args::{closure#1}::{closure#0}, core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>>
  29:     0x55933dab3f4c - rustc_span[46ba486e373c9e8b]::set_source_map::<core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>, rustc_interface[90a55842af0da270]::interface::run_compiler<core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>, rustdoc[97eab9d96eb2a784]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  30:     0x55933da69a35 - <scoped_tls[58b6126715047eae]::ScopedKey<rustc_span[46ba486e373c9e8b]::SessionGlobals>>::set::<rustc_interface[90a55842af0da270]::interface::run_compiler<core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>, rustdoc[97eab9d96eb2a784]::main_args::{closure#1}>::{closure#0}, core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>>
  31:     0x55933d9ac5a6 - std[53ab809165a7f1fe]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[90a55842af0da270]::util::run_in_thread_pool_with_globals<rustc_interface[90a55842af0da270]::interface::run_compiler<core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>, rustdoc[97eab9d96eb2a784]::main_args::{closure#1}>::{closure#0}, core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>>
  32:     0x55933da380d9 - <<std[53ab809165a7f1fe]::thread::Builder>::spawn_unchecked_<rustc_interface[90a55842af0da270]::util::run_in_thread_pool_with_globals<rustc_interface[90a55842af0da270]::interface::run_compiler<core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>, rustdoc[97eab9d96eb2a784]::main_args::{closure#1}>::{closure#0}, core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cff1766242b3081]::result::Result<(), rustc_span[46ba486e373c9e8b]::ErrorGuaranteed>>::{closure#1} as core[cff1766242b3081]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  33:     0x7f470529f49e - std::sys::unix::thread::Thread::new::thread_start::h06761b9c43e64d9e
  34:     0x7f4704f30b43 - <unknown>
  35:     0x7f4704fc2a00 - <unknown>
  36:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

---
---- [rustdoc] tests/rustdoc/synthetic_auto/complex.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/complex" "/checkout/tests/rustdoc/synthetic_auto/complex.rs"
stdout: none
23: @has check failed
23: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl"]//h3[@class="code-header"]' "impl<'a, T, K: ?Sized> Send for Outer<'a, T, K>where K: for<'b> Fn((&'b bool, &'a u8)) -> &'b i8, T: MyTrait<'a>, <T as MyTrait<'a>>::MyItem: Copy, 'a: 'static"
Encountered 1 errors
------------------------------------------


