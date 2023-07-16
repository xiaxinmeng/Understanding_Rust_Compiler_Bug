
thread 'rustc' panicked at 'forcing query with already existing `DepNode`
- query-key: Binder(cluster_set::ClusterInternal<{integer}, ()>, [])
- dep-node: own_existential_vtable_entries(a965f9c82c7a057e-34da6f267b48538f)', /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/compiler/rustc_query_system/src/dep_graph/graph.rs:247:9
stack backtrace:
[...]
  10:        0x10e3bb05f - core::panicking::panic_fmt::hb64a2db862b4aca0
  11:        0x117da5f52 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h14823017a5a1697e
  12:        0x117cd5f30 - rustc_data_structures::stack::ensure_sufficient_stack::haba6544b1a98446b
  13:        0x117b82975 - rustc_query_system::query::plumbing::try_execute_query::hd6604657c722c436
  14:        0x117c116fb - rustc_query_system::query::plumbing::get_query::hbda43b871f1e68ac
  15:        0x117d4f06c - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::own_existential_vtable_entries::habf6e1b1127c9ae1
  16:        0x11878aa71 - rustc_trait_selection::traits::util::count_own_vtable_entries::hf252515eecb86653
  17:        0x1187581b9 - rustc_trait_selection::traits::vtable_trait_first_method_offset::h0886bae50e69c0ef
  18:        0x11872d4be - rustc_trait_selection::traits::select::confirmation::<impl rustc_trait_selection::traits::select::SelectionContext>::confirm_candidate::hb5c10dc807d780d5
[...]
