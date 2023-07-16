plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 585 tests
i...............F.........................F.......F.................i................... 88/585
.....F....................................F.....F.F.....................F............... 176/585
........F.F..FFFF.......F....F.F..F............F...............................F........ 264/585
.....................................................................F..F..........F..F. 352/585
.....................................F.F....FF.i.F.............................F....FF.. 440/585
.....................F.....F.F..F.........iFFF...........F.............................. 528/585
...............................................F.F.......

---- [rustdoc] src/test/rustdoc/cfg_doc_reexport.rs stdout ----

error: rustdoc failed!
error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cfg_doc_reexport/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cfg_doc_reexport" "--deny" "warnings" "/checkout/src/test/rustdoc/cfg_doc_reexport.rs"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/query.rs:393:1: `tcx.item_attrs(DefId(0:7 ~ foo[fddf]::bar::Bar))` is not supported for local crate;
                                
                                                        hint: Queries can be either made to the local crate, or the external crate. This error means you tried to use it for one that's not supported.
                                
                                                        If that's not the case, item_attrs was likely never assigned to a provider function.
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1587:9
stack backtrace:
stack backtrace:
   0:     0x7f0d21ff2215 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed3a476cb5386522
   1:     0x7f0d22062468 - core::fmt::write::hedfeb44df60aa14d
   2:     0x7f0d21fe42d1 - std::io::Write::write_fmt::hc5bdb0d5d7f008f8
   3:     0x7f0d21ff2021 - std::sys_common::backtrace::print::hfbda0d631907f8a4
   4:     0x7f0d21ff5404 - std::panicking::default_hook::{{closure}}::h6b86a79b22dd4502
   5:     0x7f0d21ff50ca - std::panicking::default_hook::had9d4f0f6fea3822
   6:     0x7f0d22a3e9c2 - rustc_driver[2b9d07669eebebe9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f0d21ff5b74 - std::panicking::rust_panic_with_hook::h7355c621e6a8e903
   8:     0x7f0d2581fd13 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}
   9:     0x7f0d2580b8f6 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_end_short_backtrace::<std[fd153d55f3bd48c6]::panicking::begin_panic<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}, !>
  10:     0x7f0d229d8bc6 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  11:     0x7f0d2580b246 - std[fd153d55f3bd48c6]::panic::panic_any::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  12:     0x7f0d25802880 - <rustc_errors[5d16906d1a7d0872]::HandlerInner>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  13:     0x7f0d25800070 - <rustc_errors[5d16906d1a7d0872]::Handler>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  14:     0x7f0d2589b685 - rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_context_opt::<rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_opt<rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f0d258a4219 - rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt::<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>
  16:     0x7f0d229e60e8 - rustc_middle[f13816ef3642e9c4]::util::bug::bug_fmt
  17:     0x7f0d256ff051 - <<rustc_middle[f13816ef3642e9c4]::ty::query::Providers as core[36c44e25ea40240]::default::Default>::default::{closure#144} as core[36c44e25ea40240]::ops::function::FnOnce<(rustc_middle[f13816ef3642e9c4]::ty::context::TyCtxt, rustc_span[9c1db07c3f5bf223]::def_id::DefId)>>::call_once
  18:     0x7f0d247d475c - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::try_execute_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt>
  19:     0x7f0d248c50f6 - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::get_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt, rustc_middle[f13816ef3642e9c4]::dep_graph::dep_node::DepKind>
  20:     0x7f0d2450fa43 - <rustc_query_impl[22c4988ac720fac8]::Queries as rustc_middle[f13816ef3642e9c4]::ty::query::QueryEngine>::item_attrs
  21:     0x55818b99beb6 - <rustdoc[89fe7831682a6590]::clean::types::Import>::imported_item_is_doc_hidden
  22:     0x55818b9f97e1 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  23:     0x55818bb585ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  24:     0x55818b9e1cd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  25:     0x55818b9f998f - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  26:     0x55818b9e2d41 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_crate
  27:     0x55818b90e7d7 - rustdoc[89fe7831682a6590]::passes::strip_private::strip_private
  28:     0x55818ba2508e - <rustc_session[467a6c2f6f3528be]::session::Session>::time::<rustdoc[89fe7831682a6590]::clean::types::Crate, rustdoc[89fe7831682a6590]::core::run_global_ctxt::{closure#7}>
  29:     0x55818b9a12af - rustdoc[89fe7831682a6590]::core::run_global_ctxt
  30:     0x55818ba29b30 - <rustc_interface[2d03d6d32bb84688]::passes::QueryContext>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}::{closure#1}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  31:     0x55818bba8df3 - <rustc_interface[2d03d6d32bb84688]::interface::Compiler>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  32:     0x55818b96b86e - rustc_span[9c1db07c3f5bf223]::with_source_map::<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  33:     0x55818b8d3c14 - <scoped_tls[8fd4ed7a9e246a42]::ScopedKey<rustc_span[9c1db07c3f5bf223]::SessionGlobals>>::set::<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  34:     0x55818b978349 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  35:     0x55818b926358 - std[fd153d55f3bd48c6]::panic::catch_unwind::<core[36c44e25ea40240]::panic::unwind_safe::AssertUnwindSafe<<std[fd153d55f3bd48c6]::thread::Builder>::spawn_unchecked_<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  36:     0x55818bac6587 - <<std[fd153d55f3bd48c6]::thread::Builder>::spawn_unchecked_<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#1} as core[36c44e25ea40240]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7f0d2200297e - std::sys::unix::thread::Thread::new::thread_start::he69e101c8353f5ec
  38:     0x7f0d21c8eb43 - <unknown>
  39:     0x7f0d21d20a00 - <unknown>
  40:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (eed888e34 2023-01-06) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [item_attrs] collecting attributes of `bar::Bar`
error: aborting due to previous error
------------------------------------------



---- [rustdoc] src/test/rustdoc/check-source-code-urls-to-def.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/check-source-code-urls-to-def/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/check-source-code-urls-to-def" "--deny" "warnings" "/checkout/src/test/rustdoc/check-source-code-urls-to-def.rs" "-Zunstable-options" "--generate-link-to-definition"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/query.rs:393:1: `tcx.item_attrs(DefId(0:17 ~ foo[fddf]::Foo))` is not supported for local crate;
                                
                                                        hint: Queries can be either made to the local crate, or the external crate. This error means you tried to use it for one that's not supported.
                                
                                                        If that's not the case, item_attrs was likely never assigned to a provider function.
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1587:9
stack backtrace:
stack backtrace:
   0:     0x7fefba4aa215 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed3a476cb5386522
   1:     0x7fefba51a468 - core::fmt::write::hedfeb44df60aa14d
   2:     0x7fefba49c2d1 - std::io::Write::write_fmt::hc5bdb0d5d7f008f8
   3:     0x7fefba4aa021 - std::sys_common::backtrace::print::hfbda0d631907f8a4
   4:     0x7fefba4ad404 - std::panicking::default_hook::{{closure}}::h6b86a79b22dd4502
   5:     0x7fefba4ad0ca - std::panicking::default_hook::had9d4f0f6fea3822
   6:     0x7fefbaef69c2 - rustc_driver[2b9d07669eebebe9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fefba4adb74 - std::panicking::rust_panic_with_hook::h7355c621e6a8e903
   8:     0x7fefbdcd7d13 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}
   9:     0x7fefbdcc38f6 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_end_short_backtrace::<std[fd153d55f3bd48c6]::panicking::begin_panic<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}, !>
  10:     0x7fefbae90bc6 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  11:     0x7fefbdcc3246 - std[fd153d55f3bd48c6]::panic::panic_any::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  12:     0x7fefbdcba880 - <rustc_errors[5d16906d1a7d0872]::HandlerInner>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  13:     0x7fefbdcb8070 - <rustc_errors[5d16906d1a7d0872]::Handler>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  14:     0x7fefbdd53685 - rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_context_opt::<rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_opt<rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7fefbdd5c219 - rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt::<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>
  16:     0x7fefbae9e0e8 - rustc_middle[f13816ef3642e9c4]::util::bug::bug_fmt
  17:     0x7fefbdbb7051 - <<rustc_middle[f13816ef3642e9c4]::ty::query::Providers as core[36c44e25ea40240]::default::Default>::default::{closure#144} as core[36c44e25ea40240]::ops::function::FnOnce<(rustc_middle[f13816ef3642e9c4]::ty::context::TyCtxt, rustc_span[9c1db07c3f5bf223]::def_id::DefId)>>::call_once
  18:     0x7fefbcc8c75c - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::try_execute_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt>
  19:     0x7fefbcd7d0f6 - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::get_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt, rustc_middle[f13816ef3642e9c4]::dep_graph::dep_node::DepKind>
  20:     0x7fefbc9c7a43 - <rustc_query_impl[22c4988ac720fac8]::Queries as rustc_middle[f13816ef3642e9c4]::ty::query::QueryEngine>::item_attrs
  21:     0x561f91404eb6 - <rustdoc[89fe7831682a6590]::clean::types::Import>::imported_item_is_doc_hidden
  22:     0x561f914627e1 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  23:     0x561f915c15ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  24:     0x561f9144acd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  25:     0x561f9146298f - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  26:     0x561f915c15ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  27:     0x561f9144acd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  28:     0x561f9146298f - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  29:     0x561f9144bd41 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_crate
  30:     0x561f913777d7 - rustdoc[89fe7831682a6590]::passes::strip_private::strip_private
  31:     0x561f9148e08e - <rustc_session[467a6c2f6f3528be]::session::Session>::time::<rustdoc[89fe7831682a6590]::clean::types::Crate, rustdoc[89fe7831682a6590]::core::run_global_ctxt::{closure#7}>
  32:     0x561f9140a2af - rustdoc[89fe7831682a6590]::core::run_global_ctxt
  33:     0x561f91492b30 - <rustc_interface[2d03d6d32bb84688]::passes::QueryContext>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}::{closure#1}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  34:     0x561f91611df3 - <rustc_interface[2d03d6d32bb84688]::interface::Compiler>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  35:     0x561f913d486e - rustc_span[9c1db07c3f5bf223]::with_source_map::<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  36:     0x561f9133cc14 - <scoped_tls[8fd4ed7a9e246a42]::ScopedKey<rustc_span[9c1db07c3f5bf223]::SessionGlobals>>::set::<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  37:     0x561f913e1349 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  38:     0x561f9138f358 - std[fd153d55f3bd48c6]::panic::catch_unwind::<core[36c44e25ea40240]::panic::unwind_safe::AssertUnwindSafe<<std[fd153d55f3bd48c6]::thread::Builder>::spawn_unchecked_<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  39:     0x561f9152f587 - <<std[fd153d55f3bd48c6]::thread::Builder>::spawn_unchecked_<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#1} as core[36c44e25ea40240]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7fefba4ba97e - std::sys::unix::thread::Thread::new::thread_start::he69e101c8353f5ec
  41:     0x7fefba146b43 - <unknown>
  42:     0x7fefba1d8a00 - <unknown>
  43:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (eed888e34 2023-01-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unstable-options

query stack during panic:
#0 [item_attrs] collecting attributes of `Foo`
error: aborting due to previous error
------------------------------------------



---- [rustdoc] src/test/rustdoc/constructor-imports.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/constructor-imports/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/constructor-imports" "--deny" "warnings" "/checkout/src/test/rustdoc/constructor-imports.rs"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/query.rs:393:1: `tcx.item_attrs(DefId(0:4 ~ foo[fddf]::a::Foo))` is not supported for local crate;
                                
                                                        hint: Queries can be either made to the local crate, or the external crate. This error means you tried to use it for one that's not supported.
                                
                                                        If that's not the case, item_attrs was likely never assigned to a provider function.
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1587:9
stack backtrace:
   0:     0x7f6a8f74d215 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed3a476cb5386522
   1:     0x7f6a8f7bd468 - core::fmt::write::hedfeb44df60aa14d
   1:     0x7f6a8f7bd468 - core::fmt::write::hedfeb44df60aa14d
   2:     0x7f6a8f73f2d1 - std::io::Write::write_fmt::hc5bdb0d5d7f008f8
   3:     0x7f6a8f74d021 - std::sys_common::backtrace::print::hfbda0d631907f8a4
   4:     0x7f6a8f750404 - std::panicking::default_hook::{{closure}}::h6b86a79b22dd4502
   5:     0x7f6a8f7500ca - std::panicking::default_hook::had9d4f0f6fea3822
   6:     0x7f6a901999c2 - rustc_driver[2b9d07669eebebe9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f6a8f750b74 - std::panicking::rust_panic_with_hook::h7355c621e6a8e903
   8:     0x7f6a92f7ad13 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}
   9:     0x7f6a92f668f6 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_end_short_backtrace::<std[fd153d55f3bd48c6]::panicking::begin_panic<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}, !>
  10:     0x7f6a90133bc6 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  11:     0x7f6a92f66246 - std[fd153d55f3bd48c6]::panic::panic_any::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  12:     0x7f6a92f5d880 - <rustc_errors[5d16906d1a7d0872]::HandlerInner>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  13:     0x7f6a92f5b070 - <rustc_errors[5d16906d1a7d0872]::Handler>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  14:     0x7f6a92ff6685 - rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_context_opt::<rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_opt<rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f6a92fff219 - rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt::<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>
  16:     0x7f6a901410e8 - rustc_middle[f13816ef3642e9c4]::util::bug::bug_fmt
  17:     0x7f6a92e5a051 - <<rustc_middle[f13816ef3642e9c4]::ty::query::Providers as core[36c44e25ea40240]::default::Default>::default::{closure#144} as core[36c44e25ea40240]::ops::function::FnOnce<(rustc_middle[f13816ef3642e9c4]::ty::context::TyCtxt, rustc_span[9c1db07c3f5bf223]::def_id::DefId)>>::call_once
  18:     0x7f6a91f2f75c - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::try_execute_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt>
  19:     0x7f6a920200f6 - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::get_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt, rustc_middle[f13816ef3642e9c4]::dep_graph::dep_node::DepKind>
  20:     0x7f6a91c6aa43 - <rustc_query_impl[22c4988ac720fac8]::Queries as rustc_middle[f13816ef3642e9c4]::ty::query::QueryEngine>::item_attrs
  21:     0x558a8ae19eb6 - <rustdoc[89fe7831682a6590]::clean::types::Import>::imported_item_is_doc_hidden
  22:     0x558a8ae777e1 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  23:     0x558a8afd65ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  24:     0x558a8ae5fcd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  25:     0x558a8ae7798f - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  26:     0x558a8ae60d41 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_crate
  27:     0x558a8ad8c7d7 - rustdoc[89fe7831682a6590]::passes::strip_private::strip_private
  28:     0x558a8aea308e - <rustc_session[467a6c2f6f3528be]::session::Session>::time::<rustdoc[89fe7831682a6590]::clean::types::Crate, rustdoc[89fe7831682a6590]::core::run_global_ctxt::{closure#7}>
  29:     0x558a8ae1f2af - rustdoc[89fe7831682a6590]::core::run_global_ctxt
  30:     0x558a8aea7b30 - <rustc_interface[2d03d6d32bb84688]::passes::QueryContext>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}::{closure#1}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  31:     0x558a8b026df3 - <rustc_interface[2d03d6d32bb84688]::interface::Compiler>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  32:     0x558a8ade986e - rustc_span[9c1db07c3f5bf223]::with_source_map::<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  33:     0x558a8ad51c14 - <scoped_tls[8fd4ed7a9e246a42]::ScopedKey<rustc_span[9c1db07c3f5bf223]::SessionGlobals>>::set::<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  34:     0x558a8adf6349 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  35:     0x558a8ada4358 - std[fd153d55f3bd48c6]::panic::catch_unwind::<core[36c44e25ea40240]::panic::unwind_safe::AssertUnwindSafe<<std[fd153d55f3bd48c6]::thread::Builder>::spawn_unchecked_<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  36:     0x558a8af44587 - <<std[fd153d55f3bd48c6]::thread::Builder>::spawn_unchecked_<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#1} as core[36c44e25ea40240]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7f6a8f75d97e - std::sys::unix::thread::Thread::new::thread_start::he69e101c8353f5ec
  38:     0x7f6a8f3e9b43 - <unknown>
  39:     0x7f6a8f47ba00 - <unknown>
  40:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (eed888e34 2023-01-06) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [item_attrs] collecting attributes of `a::Foo`
error: aborting due to previous error
------------------------------------------



---- [rustdoc] src/test/rustdoc/doc_auto_cfg_nested_impl.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc_auto_cfg_nested_impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc_auto_cfg_nested_impl" "--deny" "warnings" "/checkout/src/test/rustdoc/doc_auto_cfg_nested_impl.rs"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/query.rs:393:1: `tcx.item_attrs(DefId(0:3 ~ foo[fddf]::S))` is not supported for local crate;
                                
                                                        hint: Queries can be either made to the local crate, or the external crate. This error means you tried to use it for one that's not supported.
                                
                                                        If that's not the case, item_attrs was likely never assigned to a provider function.
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1587:9
stack backtrace:
   0:     0x7fa95d2bf215 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed3a476cb5386522
   1:     0x7fa95d32f468 - core::fmt::write::hedfeb44df60aa14d
   1:     0x7fa95d32f468 - core::fmt::write::hedfeb44df60aa14d
   2:     0x7fa95d2b12d1 - std::io::Write::write_fmt::hc5bdb0d5d7f008f8
   3:     0x7fa95d2bf021 - std::sys_common::backtrace::print::hfbda0d631907f8a4
   4:     0x7fa95d2c2404 - std::panicking::default_hook::{{closure}}::h6b86a79b22dd4502
   5:     0x7fa95d2c20ca - std::panicking::default_hook::had9d4f0f6fea3822
   6:     0x7fa95dd0b9c2 - rustc_driver[2b9d07669eebebe9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa95d2c2b74 - std::panicking::rust_panic_with_hook::h7355c621e6a8e903
   8:     0x7fa960aecd13 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}
   9:     0x7fa960ad88f6 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_end_short_backtrace::<std[fd153d55f3bd48c6]::panicking::begin_panic<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}, !>
  10:     0x7fa95dca5bc6 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  11:     0x7fa960ad8246 - std[fd153d55f3bd48c6]::panic::panic_any::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  12:     0x7fa960acf880 - <rustc_errors[5d16906d1a7d0872]::HandlerInner>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  13:     0x7fa960acd070 - <rustc_errors[5d16906d1a7d0872]::Handler>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  14:     0x7fa960b68685 - rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_context_opt::<rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_opt<rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7fa960b71219 - rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt::<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>
  16:     0x7fa95dcb30e8 - rustc_middle[f13816ef3642e9c4]::util::bug::bug_fmt
  17:     0x7fa9609cc051 - <<rustc_middle[f13816ef3642e9c4]::ty::query::Providers as core[36c44e25ea40240]::default::Default>::default::{closure#144} as core[36c44e25ea40240]::ops::function::FnOnce<(rustc_middle[f13816ef3642e9c4]::ty::context::TyCtxt, rustc_span[9c1db07c3f5bf223]::def_id::DefId)>>::call_once
  18:     0x7fa95faa175c - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::try_execute_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt>
  19:     0x7fa95fb920f6 - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::get_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt, rustc_middle[f13816ef3642e9c4]::dep_graph::dep_node::DepKind>
  20:     0x7fa95f7dca43 - <rustc_query_impl[22c4988ac720fac8]::Queries as rustc_middle[f13816ef3642e9c4]::ty::query::QueryEngine>::item_attrs
  21:     0x55bb9e886eb6 - <rustdoc[89fe7831682a6590]::clean::types::Import>::imported_item_is_doc_hidden
  22:     0x55bb9e8e47e1 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  23:     0x55bb9ea435ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  24:     0x55bb9e8cccd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  25:     0x55bb9e8e48f0 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  26:     0x55bb9ea435ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  27:     0x55bb9e8cccd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  28:     0x55bb9e8e498f - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  29:     0x55bb9e8cdd41 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_crate
  30:     0x55bb9e7f97d7 - rustdoc[89fe7831682a6590]::passes::strip_private::strip_private
  31:     0x55bb9e91008e - <rustc_session[467a6c2f6f3528be]::session::Session>::time::<rustdoc[89fe7831682a6590]::clean::types::Crate, rustdoc[89fe7831682a6590]::core::run_global_ctxt::{closure#7}>
  32:     0x55bb9e88c2af - rustdoc[89fe7831682a6590]::core::run_global_ctxt
  33:     0x55bb9e914b30 - <rustc_interface[2d03d6d32bb84688]::passes::QueryContext>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}::{closure#1}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  34:     0x55bb9ea93df3 - <rustc_interface[2d03d6d32bb84688]::interface::Compiler>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  35:     0x55bb9e85686e - rustc_span[9c1db07c3f5bf223]::with_source_map::<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  36:     0x55bb9e7bec14 - <scoped_tls[8fd4ed7a9e246a42]::ScopedKey<rustc_span[9c1db07c3f5bf223]::SessionGlobals>>::set::<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  37:     0x55bb9e863349 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  38:     0x55bb9e811358 - std[fd153d55f3bd48c6]::panic::catch_unwind::<core[36c44e25ea40240]::panic::unwind_safe::AssertUnwindSafe<<std[fd153d55f3bd48c6]::thread::Builder>::spawn_unchecked_<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  39:     0x55bb9e9b1587 - <<std[fd153d55f3bd48c6]::thread::Builder>::spawn_unchecked_<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#1} as core[36c44e25ea40240]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7fa95d2cf97e - std::sys::unix::thread::Thread::new::thread_start::he69e101c8353f5ec
  41:     0x7fa95cf5bb43 - <unknown>
  42:     0x7fa95cfeda00 - <unknown>
  43:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (eed888e34 2023-01-06) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [item_attrs] collecting attributes of `S`
error: aborting due to previous error
------------------------------------------



---- [rustdoc] src/test/rustdoc/foreigntype-reexport.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/foreigntype-reexport/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/foreigntype-reexport" "--deny" "warnings" "/checkout/src/test/rustdoc/foreigntype-reexport.rs"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/query.rs:393:1: `tcx.item_attrs(DefId(0:11 ~ foreigntype_reexport[0a72]::sub2::{extern#0}::f))` is not supported for local crate;
                                
                                                        hint: Queries can be either made to the local crate, or the external crate. This error means you tried to use it for one that's not supported.
                                
                                                        If that's not the case, item_attrs was likely never assigned to a provider function.
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1587:9
stack backtrace:
   0:     0x7f7bd7757215 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed3a476cb5386522
   1:     0x7f7bd77c7468 - core::fmt::write::hedfeb44df60aa14d
   1:     0x7f7bd77c7468 - core::fmt::write::hedfeb44df60aa14d
   2:     0x7f7bd77492d1 - std::io::Write::write_fmt::hc5bdb0d5d7f008f8
   3:     0x7f7bd7757021 - std::sys_common::backtrace::print::hfbda0d631907f8a4
   4:     0x7f7bd775a404 - std::panicking::default_hook::{{closure}}::h6b86a79b22dd4502
   5:     0x7f7bd775a0ca - std::panicking::default_hook::had9d4f0f6fea3822
   6:     0x7f7bd81a39c2 - rustc_driver[2b9d07669eebebe9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f7bd775ab74 - std::panicking::rust_panic_with_hook::h7355c621e6a8e903
   8:     0x7f7bdaf84d13 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}
   9:     0x7f7bdaf708f6 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_end_short_backtrace::<std[fd153d55f3bd48c6]::panicking::begin_panic<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}, !>
  10:     0x7f7bd813dbc6 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  11:     0x7f7bdaf70246 - std[fd153d55f3bd48c6]::panic::panic_any::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  12:     0x7f7bdaf67880 - <rustc_errors[5d16906d1a7d0872]::HandlerInner>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  13:     0x7f7bdaf65070 - <rustc_errors[5d16906d1a7d0872]::Handler>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  14:     0x7f7bdb000685 - rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_context_opt::<rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_opt<rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f7bdb009219 - rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt::<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>
  16:     0x7f7bd814b0e8 - rustc_middle[f13816ef3642e9c4]::util::bug::bug_fmt
  17:     0x7f7bdae64051 - <<rustc_middle[f13816ef3642e9c4]::ty::query::Providers as core[36c44e25ea40240]::default::Default>::default::{closure#144} as core[36c44e25ea40240]::ops::function::FnOnce<(rustc_middle[f13816ef3642e9c4]::ty::context::TyCtxt, rustc_span[9c1db07c3f5bf223]::def_id::DefId)>>::call_once
  18:     0x7f7bd9f3975c - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::try_execute_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt>
  19:     0x7f7bda02a0f6 - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::get_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt, rustc_middle[f13816ef3642e9c4]::dep_graph::dep_node::DepKind>
  20:     0x7f7bd9c74a43 - <rustc_query_impl[22c4988ac720fac8]::Queries as rustc_middle[f13816ef3642e9c4]::ty::query::QueryEngine>::item_attrs
  21:     0x5631ea073eb6 - <rustdoc[89fe7831682a6590]::clean::types::Import>::imported_item_is_doc_hidden
  22:     0x5631ea0d17e1 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  23:     0x5631ea2305ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  24:     0x5631ea0b9cd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  25:     0x5631ea0d198f - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  26:     0x5631ea0bad41 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_crate
  27:     0x5631e9fe67d7 - rustdoc[89fe7831682a6590]::passes::strip_private::strip_private
  28:     0x5631ea0fd08e - <rustc_session[467a6c2f6f3528be]::session::Session>::time::<rustdoc[89fe7831682a6590]::clean::types::Crate, rustdoc[89fe7831682a6590]::core::run_global_ctxt::{closure#7}>
  29:     0x5631ea0792af - rustdoc[89fe7831682a6590]::core::run_global_ctxt
  30:     0x5631ea101b30 - <rustc_interface[2d03d6d32bb84688]::passes::QueryContext>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}::{closure#1}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  31:     0x5631ea280df3 - <rustc_interface[2d03d6d32bb84688]::interface::Compiler>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  32:     0x5631ea04386e - rustc_span[9c1db07c3f5bf223]::with_source_map::<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  33:     0x5631e9fabc14 - <scoped_tls[8fd4ed7a9e246a42]::ScopedKey<rustc_span[9c1db07c3f5bf223]::SessionGlobals>>::set::<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  34:     0x5631ea050349 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  35:     0x5631e9ffe358 - std[fd153d55f3bd48c6]::panic::catch_unwind::<core[36c44e25ea40240]::panic::unwind_safe::AssertUnwindSafe<<std[fd153d55f3bd48c6]::thread::Builder>::spawn_unchecked_<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  36:     0x5631ea19e587 - <<std[fd153d55f3bd48c6]::thread::Builder>::spawn_unchecked_<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#1} as core[36c44e25ea40240]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7f7bd776797e - std::sys::unix::thread::Thread::new::thread_start::he69e101c8353f5ec
  38:     0x7f7bd73f3b43 - <unknown>
  39:     0x7f7bd7485a00 - <unknown>
  40:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (eed888e34 2023-01-06) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [item_attrs] collecting attributes of `sub2::f`
error: aborting due to previous error
------------------------------------------



---- [rustdoc] src/test/rustdoc/glob-shadowing-const.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/glob-shadowing-const/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/glob-shadowing-const" "--deny" "warnings" "/checkout/src/test/rustdoc/glob-shadowing-const.rs"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/query.rs:393:1: `tcx.item_attrs(DefId(0:3 ~ foo[fddf]::sub4))` is not supported for local crate;
                                
                                                        hint: Queries can be either made to the local crate, or the external crate. This error means you tried to use it for one that's not supported.
                                
                                                        If that's not the case, item_attrs was likely never assigned to a provider function.
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1587:9
stack backtrace:
stack backtrace:
   0:     0x7f91ca0e8215 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed3a476cb5386522
   1:     0x7f91ca158468 - core::fmt::write::hedfeb44df60aa14d
   2:     0x7f91ca0da2d1 - std::io::Write::write_fmt::hc5bdb0d5d7f008f8
   3:     0x7f91ca0e8021 - std::sys_common::backtrace::print::hfbda0d631907f8a4
   4:     0x7f91ca0eb404 - std::panicking::default_hook::{{closure}}::h6b86a79b22dd4502
   5:     0x7f91ca0eb0ca - std::panicking::default_hook::had9d4f0f6fea3822
   6:     0x7f91cab349c2 - rustc_driver[2b9d07669eebebe9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f91ca0ebb74 - std::panicking::rust_panic_with_hook::h7355c621e6a8e903
   8:     0x7f91cd915d13 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}
   9:     0x7f91cd9018f6 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_end_short_backtrace::<std[fd153d55f3bd48c6]::panicking::begin_panic<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}, !>
  10:     0x7f91caacebc6 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  11:     0x7f91cd901246 - std[fd153d55f3bd48c6]::panic::panic_any::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  12:     0x7f91cd8f8880 - <rustc_errors[5d16906d1a7d0872]::HandlerInner>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  13:     0x7f91cd8f6070 - <rustc_errors[5d16906d1a7d0872]::Handler>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  14:     0x7f91cd991685 - rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_context_opt::<rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_opt<rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f91cd99a219 - rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt::<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>
  16:     0x7f91caadc0e8 - rustc_middle[f13816ef3642e9c4]::util::bug::bug_fmt
  17:     0x7f91cd7f5051 - <<rustc_middle[f13816ef3642e9c4]::ty::query::Providers as core[36c44e25ea40240]::default::Default>::default::{closure#144} as core[36c44e25ea40240]::ops::function::FnOnce<(rustc_middle[f13816ef3642e9c4]::ty::context::TyCtxt, rustc_span[9c1db07c3f5bf223]::def_id::DefId)>>::call_once
  18:     0x7f91cc8ca75c - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::try_execute_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt>
  19:     0x7f91cc9bb0f6 - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::get_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt, rustc_middle[f13816ef3642e9c4]::dep_graph::dep_node::DepKind>
  20:     0x7f91cc605a43 - <rustc_query_impl[22c4988ac720fac8]::Queries as rustc_middle[f13816ef3642e9c4]::ty::query::QueryEngine>::item_attrs
  21:     0x55607712ceb6 - <rustdoc[89fe7831682a6590]::clean::types::Import>::imported_item_is_doc_hidden
  22:     0x55607718a7e1 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  23:     0x5560772e95ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  24:     0x556077172cd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  25:     0x55607718a98f - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  26:     0x5560772e95ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  27:     0x556077172cd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  28:     0x55607718a8f0 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  29:     0x5560772e95ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  30:     0x556077172cd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  31:     0x55607718a98f - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  32:     0x556077173d41 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_crate
  33:     0x55607709f7d7 - rustdoc[89fe7831682a6590]::passes::strip_private::strip_private
  34:     0x5560771b608e - <rustc_session[467a6c2f6f3528be]::session::Session>::time::<rustdoc[89fe7831682a6590]::clean::types::Crate, rustdoc[89fe7831682a6590]::core::run_global_ctxt::{closure#7}>
  35:     0x5560771322af - rustdoc[89fe7831682a6590]::core::run_global_ctxt
  36:     0x5560771bab30 - <rustc_interface[2d03d6d32bb84688]::passes::QueryContext>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}::{closure#1}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  37:     0x556077339df3 - <rustc_interface[2d03d6d32bb84688]::interface::Compiler>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  38:     0x5560770fc86e - rustc_span[9c1db07c3f5bf223]::with_source_map::<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  39:     0x556077064c14 - <scoped_tls[8fd4ed7a9e246a42]::ScopedKey<rustc_span[9c1db07c3f5bf223]::SessionGlobals>>::set::<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  40:     0x556077109349 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  41:     0x5560770b7358 - std[fd153d55f3bd48c6]::panic::catch_unwind::<core[36c44e25ea40240]::panic::unwind_safe::AssertUnwindSafe<<std[fd153d55f3bd48c6]::thread::Builder>::spawn_unchecked_<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  42:     0x556077257587 - <<std[fd153d55f3bd48c6]::thread::Builder>::spawn_unchecked_<rustc_interface[2d03d6d32bb84688]::util::run_in_thread_pool_with_globals<rustc_interface[2d03d6d32bb84688]::interface::run_compiler<core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>, rustdoc[89fe7831682a6590]::main_args::{closure#1}>::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>::{closure#1} as core[36c44e25ea40240]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  43:     0x7f91ca0f897e - std::sys::unix::thread::Thread::new::thread_start::he69e101c8353f5ec
  44:     0x7f91c9d84b43 - <unknown>
  45:     0x7f91c9e16a00 - <unknown>
  46:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (eed888e34 2023-01-06) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [item_attrs] collecting attributes of `sub4`
error: aborting due to previous error
------------------------------------------



---- [rustdoc] src/test/rustdoc/glob-shadowing.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/glob-shadowing/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/glob-shadowing" "--deny" "warnings" "/checkout/src/test/rustdoc/glob-shadowing.rs"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/query.rs:393:1: `tcx.item_attrs(DefId(0:13 ~ glob_shadowing[78cb]::sub4))` is not supported for local crate;
                                
                                                        hint: Queries can be either made to the local crate, or the external crate. This error means you tried to use it for one that's not supported.
                                
                                                        If that's not the case, item_attrs was likely never assigned to a provider function.
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1587:9
stack backtrace:
   0:     0x7f36cd577215 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed3a476cb5386522
   1:     0x7f36cd5e7468 - core::fmt::write::hedfeb44df60aa14d
   1:     0x7f36cd5e7468 - core::fmt::write::hedfeb44df60aa14d
   2:     0x7f36cd5692d1 - std::io::Write::write_fmt::hc5bdb0d5d7f008f8
   3:     0x7f36cd577021 - std::sys_common::backtrace::print::hfbda0d631907f8a4
   4:     0x7f36cd57a404 - std::panicking::default_hook::{{closure}}::h6b86a79b22dd4502
   5:     0x7f36cd57a0ca - std::panicking::default_hook::had9d4f0f6fea3822
   6:     0x7f36cdfc39c2 - rustc_driver[2b9d07669eebebe9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f36cd57ab74 - std::panicking::rust_panic_with_hook::h7355c621e6a8e903
   8:     0x7f36d0da4d13 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}
   9:     0x7f36d0d908f6 - std[fd153d55f3bd48c6]::sys_common::backtrace::__rust_end_short_backtrace::<std[fd153d55f3bd48c6]::panicking::begin_panic<rustc_errors[5d16906d1a7d0872]::ExplicitBug>::{closure#0}, !>
  10:     0x7f36cdf5dbc6 - std[fd153d55f3bd48c6]::panicking::begin_panic::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  11:     0x7f36d0d90246 - std[fd153d55f3bd48c6]::panic::panic_any::<rustc_errors[5d16906d1a7d0872]::ExplicitBug>
  12:     0x7f36d0d87880 - <rustc_errors[5d16906d1a7d0872]::HandlerInner>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  13:     0x7f36d0d85070 - <rustc_errors[5d16906d1a7d0872]::Handler>::bug::<&alloc[e0cdd4263fde2a6c]::string::String>
  14:     0x7f36d0e20685 - rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_context_opt::<rustc_middle[f13816ef3642e9c4]::ty::context::tls::with_opt<rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f36d0e29219 - rustc_middle[f13816ef3642e9c4]::util::bug::opt_span_bug_fmt::<rustc_span[9c1db07c3f5bf223]::span_encoding::Span>
  16:     0x7f36cdf6b0e8 - rustc_middle[f13816ef3642e9c4]::util::bug::bug_fmt
  17:     0x7f36d0c84051 - <<rustc_middle[f13816ef3642e9c4]::ty::query::Providers as core[36c44e25ea40240]::default::Default>::default::{closure#144} as core[36c44e25ea40240]::ops::function::FnOnce<(rustc_middle[f13816ef3642e9c4]::ty::context::TyCtxt, rustc_span[9c1db07c3f5bf223]::def_id::DefId)>>::call_once
  18:     0x7f36cfd5975c - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::try_execute_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt>
  19:     0x7f36cfe4a0f6 - rustc_query_system[b701edc3c2fd1f1e]::query::plumbing::get_query::<rustc_query_impl[22c4988ac720fac8]::queries::item_attrs, rustc_query_impl[22c4988ac720fac8]::plumbing::QueryCtxt, rustc_middle[f13816ef3642e9c4]::dep_graph::dep_node::DepKind>
  20:     0x7f36cfa94a43 - <rustc_query_impl[22c4988ac720fac8]::Queries as rustc_middle[f13816ef3642e9c4]::ty::query::QueryEngine>::item_attrs
  21:     0x560a6de7beb6 - <rustdoc[89fe7831682a6590]::clean::types::Import>::imported_item_is_doc_hidden
  22:     0x560a6ded97e1 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  23:     0x560a6e0385ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  24:     0x560a6dec1cd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  25:     0x560a6ded998f - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  26:     0x560a6e0385ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  27:     0x560a6dec1cd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  28:     0x560a6ded98f0 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  29:     0x560a6e0385ab - <alloc[e0cdd4263fde2a6c]::vec::Vec<rustdoc[89fe7831682a6590]::clean::types::Item> as alloc[e0cdd4263fde2a6c]::vec::spec_from_iter::SpecFromIter<rustdoc[89fe7831682a6590]::clean::types::Item, core[36c44e25ea40240]::iter::adapters::filter_map::FilterMap<alloc[e0cdd4263fde2a6c]::vec::into_iter::IntoIter<rustdoc[89fe7831682a6590]::clean::types::Item>, <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur::{closure#3}>>>::from_iter
  30:     0x560a6dec1cd8 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_inner_recur
  31:     0x560a6ded998f - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_item
  32:     0x560a6dec2d41 - <rustdoc[89fe7831682a6590]::passes::stripper::ImportStripper as rustdoc[89fe7831682a6590]::fold::DocFolder>::fold_crate
  33:     0x560a6ddee7d7 - rustdoc[89fe7831682a6590]::passes::strip_private::strip_private
  34:     0x560a6df0508e - <rustc_session[467a6c2f6f3528be]::session::Session>::time::<rustdoc[89fe7831682a6590]::clean::types::Crate, rustdoc[89fe7831682a6590]::core::run_global_ctxt::{closure#7}>
  35:     0x560a6de812af - rustdoc[89fe7831682a6590]::core::run_global_ctxt
  36:     0x560a6df09b30 - <rustc_interface[2d03d6d32bb84688]::passes::QueryContext>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}::{closure#1}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
  37:     0x560a6e088df3 - <rustc_interface[2d03d6d32bb84688]::interface::Compiler>::enter::<rustdoc[89fe7831682a6590]::main_args::{closure#1}::{closure#0}, core[36c44e25ea40240]::result::Result<(), rustc_errors[5d16906d1a7d0872]::ErrorGuaranteed>>
