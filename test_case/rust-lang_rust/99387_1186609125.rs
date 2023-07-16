
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed
  |
  = note: delayed at    0: <rustc_errors::HandlerInner>::delay_good_path_bug::<&str>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/lib.rs:1364:25
             1: <rustc_errors::Handler>::delay_good_path_bug::<&str>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/lib.rs:927:9
             2: rustc_middle::ty::print::pretty::trimmed_def_paths
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/print/pretty.rs:2714:9
             3: <rustc_query_system::query::config::QueryVtable<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::compute
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/config.rs:43:9
             4: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:384:55
             5: stacker::maybe_grow::<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>
                       at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
             6: rustc_data_structures::stack::ensure_sufficient_stack::<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
             7: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:112:17
             8: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1930:50
             9: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1914:9
            10: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1930:9
            11: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:111:13
            12: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1974:13
            13: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1958:40
            14: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1947:22
            15: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1958:9
            16: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1971:9
            17: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:100:9
            18: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:384:22
            19: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:343:44
            20: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:702:36
            21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::trimmed_def_paths
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/lib.rs:56:1
            22: <rustc_middle::ty::query::TyCtxtAt>::trimmed_def_paths
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/query.rs:258:17
            23: <rustc_middle::ty::context::TyCtxt>::trimmed_def_paths
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/query.rs:239:17
            24: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::try_print_trimmed_def_path
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/print/pretty.rs:308:15
            25: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/print/pretty.rs:1656:19
            26: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::default_print_def_path::{closure#2}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/print/mod.rs:154:42
            27: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::path_generic_args::<<rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::default_print_def_path::{closure#2}>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/print/pretty.rs:1819:16
            28: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::default_print_def_path
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/print/mod.rs:153:40
            29: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/print/pretty.rs:1704:9
            30: <rustc_middle::ty::print::pretty::TraitRefPrintOnlyTraitPath as rustc_middle::ty::print::Print<rustc_middle::ty::print::pretty::FmtPrinter>>::print
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/print/pretty.rs:2520:9
            31: <rustc_middle::ty::print::pretty::TraitRefPrintOnlyTraitPath as core::fmt::Display>::fmt::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/print/pretty.rs:2354:30
            32: rustc_middle::ty::context::tls::with::<<rustc_middle::ty::print::pretty::TraitRefPrintOnlyTraitPath as core::fmt::Display>::fmt::{closure#0}, core::result::Result<(), core::fmt::Error>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1985:32
            33: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with<<rustc_middle::ty::print::pretty::TraitRefPrintOnlyTraitPath as core::fmt::Display>::fmt::{closure#0}, core::result::Result<(), core::fmt::Error>>::{closure#0}, core::result::Result<(), core::fmt::Error>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1958:40
            34: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with<<rustc_middle::ty::print::pretty::TraitRefPrintOnlyTraitPath as core::fmt::Display>::fmt::{closure#0}, core::result::Result<(), core::fmt::Error>>::{closure#0}, core::result::Result<(), core::fmt::Error>>::{closure#0}, core::result::Result<(), core::fmt::Error>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1947:22
            35: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with<<rustc_middle::ty::print::pretty::TraitRefPrintOnlyTraitPath as core::fmt::Display>::fmt::{closure#0}, core::result::Result<(), core::fmt::Error>>::{closure#0}, core::result::Result<(), core::fmt::Error>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1958:9
            36: rustc_middle::ty::context::tls::with::<<rustc_middle::ty::print::pretty::TraitRefPrintOnlyTraitPath as core::fmt::Display>::fmt::{closure#0}, core::result::Result<(), core::fmt::Error>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1985:9
            37: <rustc_middle::ty::print::pretty::TraitRefPrintOnlyTraitPath as core::fmt::Display>::fmt
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/print/pretty.rs:2353:17
            38: <dyn core::fmt::Display as alloc::string::ToString>::to_string
                       at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/string.rs:2489:9
            39: <rustc_privacy::SearchInterfaceForPrivateItemsVisitor>::check_def_id
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_privacy/src/lib.rs:1752:25
            40: <rustc_privacy::SearchInterfaceForPrivateItemsVisitor as rustc_privacy::DefIdVisitor>::visit_def_id
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_privacy/src/lib.rs:1812:12
            41: <rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>::visit_trait
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_privacy/src/lib.rs:114:9
            42: <rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>::visit_projection_ty
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_privacy/src/lib.rs:124:9
            43: <rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor> as rustc_middle::ty::visit::TypeVisitor>::visit_ty
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_privacy/src/lib.rs:230:24
            44: <rustc_middle::ty::Ty as rustc_middle::ty::visit::TypeVisitable>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/structural_impls.rs:993:9
            45: <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::visit::TypeVisitable>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/subst.rs:495:38
            46: core::iter::traits::iterator::Iterator::try_for_each::call::<rustc_middle::ty::Ty, core::ops::control_flow::ControlFlow<()>, <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::visit::TypeVisitable>::visit_with<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>::{closure#0}>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/iter/traits/iterator.rs:2296:26
            47: core::iter::adapters::copied::copy_try_fold::<rustc_middle::ty::Ty, (), core::ops::control_flow::ControlFlow<()>, core::iter::traits::iterator::Iterator::try_for_each::call<rustc_middle::ty::Ty, core::ops::control_flow::ControlFlow<()>, <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::visit::TypeVisitable>::visit_with<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>::{closure#0}>::{closure#0}>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/iter/adapters/copied.rs:32:22
            48: <core::slice::iter::Iter<rustc_middle::ty::Ty> as core::iter::traits::iterator::Iterator>::try_fold::<(), core::iter::adapters::copied::copy_try_fold<rustc_middle::ty::Ty, (), core::ops::control_flow::ControlFlow<()>, core::iter::traits::iterator::Iterator::try_for_each::call<rustc_middle::ty::Ty, core::ops::control_flow::ControlFlow<()>, <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::visit::TypeVisitable>::visit_with<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>::{closure#0}>::{closure#0}>::{closure#0}, core::ops::control_flow::ControlFlow<()>>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/iter/traits/iterator.rs:2238:21
            49: <core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::Ty>> as core::iter::traits::iterator::Iterator>::try_fold::<(), core::iter::traits::iterator::Iterator::try_for_each::call<rustc_middle::ty::Ty, core::ops::control_flow::ControlFlow<()>, <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::visit::TypeVisitable>::visit_with<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>::{closure#0}>::{closure#0}, core::ops::control_flow::ControlFlow<()>>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/iter/adapters/copied.rs:57:9
            50: <core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::Ty>> as core::iter::traits::iterator::Iterator>::try_for_each::<<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::visit::TypeVisitable>::visit_with<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>::{closure#0}, core::ops::control_flow::ControlFlow<()>>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/iter/traits/iterator.rs:2299:9
            51: <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::visit::TypeVisitable>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/subst.rs:495:9
            52: <rustc_middle::ty::sty::FnSig as rustc_middle::ty::visit::TypeVisitable>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/sty.rs:1215:36
            53: <rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig> as rustc_middle::ty::visit::TypeSuperVisitable>::super_visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/structural_impls.rs:893:9
            54: <rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor> as rustc_middle::ty::visit::TypeVisitor>::visit_binder::<rustc_middle::ty::sty::FnSig>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/visit.rs:187:9
            55: <rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig> as rustc_middle::ty::visit::TypeVisitable>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/structural_impls.rs:878:9
            56: <rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor> as rustc_middle::ty::visit::TypeVisitor>::visit_ty
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_privacy/src/lib.rs:208:21
            57: <rustc_middle::ty::Ty as rustc_middle::ty::visit::TypeVisitable>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/structural_impls.rs:993:9
            58: <rustc_privacy::SearchInterfaceForPrivateItemsVisitor as rustc_privacy::DefIdVisitor>::visit::<rustc_middle::ty::Ty>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_privacy/src/lib.rs:83:9
            59: <rustc_privacy::SearchInterfaceForPrivateItemsVisitor>::ty
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_privacy/src/lib.rs:1713:9
            60: <rustc_privacy::PrivateItemsInPublicInterfacesChecker>::check_item
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_privacy/src/lib.rs:1867:17
            61: rustc_privacy::check_private_in_public
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_privacy/src/lib.rs:2098:9
            62: <rustc_query_system::query::config::QueryVtable<rustc_query_impl::plumbing::QueryCtxt, (), ()>>::compute
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/config.rs:43:9
            63: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:384:55
            64: stacker::maybe_grow::<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>
                       at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
            65: rustc_data_structures::stack::ensure_sufficient_stack::<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
            66: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:112:17
            67: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}::{closure#0}, ()>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1930:50
            68: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}::{closure#0}, ()>::{closure#0}, ()>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1914:9
            69: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}::{closure#0}, ()>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1930:9
            70: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:111:13
            71: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1974:13
            72: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>::{closure#0}, ()>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1958:40
            73: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>::{closure#0}, ()>::{closure#0}, ()>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1947:22
            74: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>::{closure#0}, ()>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1958:9
            75: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1971:9
            76: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:100:9
            77: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), ()>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:384:22
            78: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), ()>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:343:44
            79: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::check_private_in_public, rustc_query_impl::plumbing::QueryCtxt>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:702:36
            80: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_private_in_public
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/lib.rs:56:1
            81: <rustc_middle::ty::query::TyCtxtEnsure>::check_private_in_public
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/query.rs:229:17
            82: rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:960:25
            83: <rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#0} as core::ops::function::FnOnce<()>>::call_once
                       at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ops/function.rs:248:5
            84: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
                       at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panic/unwind_safe.rs:271:9
            85: std::panicking::try::do_call::<core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#0}>, ()>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:492:40
            86: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#0}>>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:456:19
            87: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#0}>, ()>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panic.rs:137:14
            88: rustc_interface::passes::analysis::{closure#5}::{closure#1}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:958:17
            89: <rustc_interface::passes::analysis::{closure#5}::{closure#1} as core::ops::function::FnOnce<()>>::call_once
                       at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ops/function.rs:248:5
            90: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
                       at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panic/unwind_safe.rs:271:9
            91: std::panicking::try::do_call::<core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}>, ()>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:492:40
            92: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}>>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:456:19
            93: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}>, ()>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panic.rs:137:14
            94: rustc_interface::passes::analysis::{closure#5}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:954:9
            95: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<(), rustc_interface::passes::analysis::{closure#5}>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/profiling.rs:739:9
            96: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#5}>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_session/src/utils.rs:10:9
            97: rustc_interface::passes::analysis
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:953:5
            98: <rustc_query_system::query::config::QueryVtable<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>::compute
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/config.rs:43:9
            99: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:384:55
           100: stacker::maybe_grow::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>
                       at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
           101: rustc_data_structures::stack::ensure_sufficient_stack::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
           102: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:112:17
           103: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1930:50
           104: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1914:9
           105: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1930:9
           106: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:111:13
           107: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1974:13
           108: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1958:40
           109: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1947:22
           110: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1958:9
           111: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1971:9
           112: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/plumbing.rs:100:9
           113: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:384:22
           114: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:343:44
           115: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_system/src/query/plumbing.rs:702:36
           116: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_query_impl/src/lib.rs:56:1
           117: <rustc_middle::ty::query::TyCtxtAt>::analysis
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/query.rs:258:17
           118: <rustc_middle::ty::context::TyCtxt>::analysis
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/query.rs:239:17
           119: rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:379:30
           120: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:787:42
           121: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1930:50
           122: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1914:9
           123: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/ty/context.rs:1930:9
           124: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:787:9
           125: rustc_driver::run_compiler::{closure#1}::{closure#2}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:378:13
           126: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:381:19
           127: rustc_driver::run_compiler::{closure#1}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:310:22
           128: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:323:13
           129: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:986:5
           130: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:317:5
           131: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:337:12
           132: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
           133: rustc_span::create_session_globals_then::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}>
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:112:5
           134: rustc_interface::util::run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/util.rs:160:32
           135: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:122:18
           136: <std::thread::Builder>::spawn_unchecked_::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:505:17
           137: <core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
                       at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panic/unwind_safe.rs:271:9
           138: std::panicking::try::do_call::<core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:492:40
           139: std::panicking::try::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:456:19
           140: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panic.rs:137:14
           141: <std::thread::Builder>::spawn_unchecked_::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:504:30
           142: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                       at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ops/function.rs:248:5
           143: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1935:9
           144: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1935:9
           145: std::sys::unix::thread::Thread::new::thread_start
                       at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys/unix/thread.rs:108:17
           146: <unknown>
           147: clone
