
RUST_BACKTRACE=1 cargo test my_test
   Compiling my_crate v0.0.0 (my_projects/my_crate)
thread 'rustc' panicked at 'index out of bounds: the len is 977 but the index is 978', compiler/rustc_query_impl/src/on_disk_cache.rs:726:18
stack backtrace:
   0: rust_begin_unwind
             at /rustc/436b81f58dd94dec44d33945454048a76655fbda/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/436b81f58dd94dec44d33945454048a76655fbda/library/core/src/panicking.rs:107:14
   2: core::panicking::panic_bounds_check
             at /rustc/436b81f58dd94dec44d33945454048a76655fbda/library/core/src/panicking.rs:75:5
   3: <rustc_span::span_encoding::Span as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   4: <rustc_middle::ty::FieldDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   5: <rustc_middle::ty::VariantDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   6: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_seq::<alloc::vec::Vec<rustc_middle::ty::VariantDef>, <alloc::vec::Vec<rustc_middle::ty::VariantDef> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
   7: <rustc_middle::ty::adt::AdtDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   8: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   9: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  10: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  11: <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  12: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  13: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  14: <smallvec::SmallVec<[&rustc_middle::ty::TyS; 8]> as core::iter::traits::collect::Extend<&rustc_middle::ty::TyS>>::extend::<core::iter::adapters::ResultShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, alloc::string::String>>
  15: core::iter::adapters::process_results::<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, &rustc_middle::ty::TyS, alloc::string::String, <core::result::Result<smallvec::SmallVec<[&rustc_middle::ty::TyS; 8]>, alloc::string::String> as core::iter::traits::collect::FromIterator<core::result::Result<&rustc_middle::ty::TyS, alloc::string::String>>>::from_iter<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>>::{closure#0}, smallvec::SmallVec<[&rustc_middle::ty::TyS; 8]>>
  16: <core::result::Result<&rustc_middle::ty::TyS, alloc::string::String> as rustc_middle::ty::context::InternIteratorElement<&rustc_middle::ty::TyS, &rustc_middle::ty::list::List<&rustc_middle::ty::TyS>>>::intern_with::<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, <rustc_middle::ty::context::TyCtxt>::mk_type_list<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>>::{closure#0}>
  17: <&rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  18: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  19: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  20: <smallvec::SmallVec<[rustc_middle::ty::subst::GenericArg; 8]> as core::iter::traits::collect::Extend<rustc_middle::ty::subst::GenericArg>>::extend::<core::iter::adapters::ResultShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, alloc::string::String>>
  21: <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  22: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  23: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  24: <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  25: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  26: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  27: <rustc_query_impl::on_disk_cache::OnDiskCache>::try_load_query_result::<&rustc_middle::ty::TyS>
  28: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, &rustc_middle::ty::TyS>
  29: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
  30: rustc_typeck::check::check::check_item_type
  31: <rustc_middle::hir::map::Map>::visit_item_likes_in_module::<rustc_typeck::check::CheckItemTypesVisitor>
  32: rustc_typeck::check::check::check_mod_item_types
  33: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>
  34: rustc_data_structures::stack::ensure_sufficient_stack::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>
  35: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, ()>>
  36: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::check_mod_item_types, rustc_query_impl::plumbing::QueryCtxt>
  37: <rustc_middle::hir::map::Map>::for_each_module::<rustc_typeck::check_crate::{closure#6}::{closure#0}>
  38: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#6}>
  39: rustc_typeck::check_crate
  40: rustc_interface::passes::analysis
  41: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
  42: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
  43: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  44: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  45: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  46: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  47: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
  48: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-beta.4 (436b81f58 2022-01-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C link-arg=-fuse-ld=lld --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_of] computing type of `<impl at my_crate/src/lib.rs:78:1: 971:2>::my_func::{opaque#0}`
#1 [check_mod_item_types] checking item types in top-level module
#2 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `my_crate`
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'index out of bounds: the len is 977 but the index is 978', compiler/rustc_query_impl/src/on_disk_cache.rs:726:18
stack backtrace:
   0: rust_begin_unwind
             at /rustc/436b81f58dd94dec44d33945454048a76655fbda/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/436b81f58dd94dec44d33945454048a76655fbda/library/core/src/panicking.rs:107:14
   2: core::panicking::panic_bounds_check
             at /rustc/436b81f58dd94dec44d33945454048a76655fbda/library/core/src/panicking.rs:75:5
   3: <rustc_span::span_encoding::Span as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   4: <rustc_middle::ty::FieldDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   5: <rustc_middle::ty::VariantDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   6: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_seq::<alloc::vec::Vec<rustc_middle::ty::VariantDef>, <alloc::vec::Vec<rustc_middle::ty::VariantDef> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
   7: <rustc_middle::ty::adt::AdtDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   8: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   9: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  10: <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  11: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  12: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  13: <smallvec::SmallVec<[&rustc_middle::ty::TyS; 8]> as core::iter::traits::collect::Extend<&rustc_middle::ty::TyS>>::extend::<core::iter::adapters::ResultShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, alloc::string::String>>
  14: core::iter::adapters::process_results::<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, &rustc_middle::ty::TyS, alloc::string::String, <core::result::Result<smallvec::SmallVec<[&rustc_middle::ty::TyS; 8]>, alloc::string::String> as core::iter::traits::collect::FromIterator<core::result::Result<&rustc_middle::ty::TyS, alloc::string::String>>>::from_iter<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>>::{closure#0}, smallvec::SmallVec<[&rustc_middle::ty::TyS; 8]>>
  15: <core::result::Result<&rustc_middle::ty::TyS, alloc::string::String> as rustc_middle::ty::context::InternIteratorElement<&rustc_middle::ty::TyS, &rustc_middle::ty::list::List<&rustc_middle::ty::TyS>>>::intern_with::<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, <rustc_middle::ty::context::TyCtxt>::mk_type_list<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>>::{closure#0}>
  16: <&rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  17: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  18: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  19: <smallvec::SmallVec<[rustc_middle::ty::subst::GenericArg; 8]> as core::iter::traits::collect::Extend<rustc_middle::ty::subst::GenericArg>>::extend::<core::iter::adapters::ResultShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, alloc::string::String>>
  20: <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  21: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  22: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  23: <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  24: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  25: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  26: <rustc_query_impl::on_disk_cache::OnDiskCache>::try_load_query_result::<&rustc_middle::ty::TyS>
  27: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, &rustc_middle::ty::TyS>
  28: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
  29: rustc_typeck::check::check::check_item_type
  30: <rustc_middle::hir::map::Map>::visit_item_likes_in_module::<rustc_typeck::check::CheckItemTypesVisitor>
  31: rustc_typeck::check::check::check_mod_item_types
  32: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>
  33: rustc_data_structures::stack::ensure_sufficient_stack::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>
  34: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, ()>>
  35: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::check_mod_item_types, rustc_query_impl::plumbing::QueryCtxt>
  36: <rustc_middle::hir::map::Map>::for_each_module::<rustc_typeck::check_crate::{closure#6}::{closure#0}>
  37: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#6}>
  38: rustc_typeck::check_crate
  39: rustc_interface::passes::analysis
  40: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
  41: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
  42: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  43: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  44: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  45: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  46: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
  47: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-beta.4 (436b81f58 2022-01-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C link-arg=-fuse-ld=lld

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_of] computing type of `<impl at my_crate/src/lib.rs:78:1: 971:2>::my_func::{opaque#0}`
#1 [check_mod_item_types] checking item types in top-level module
#2 [analysis] running analysis passes on this crate
end of query stack
error: build failed

Compilation exited abnormally with code 101 at Mon Jan 31 12:54:08
