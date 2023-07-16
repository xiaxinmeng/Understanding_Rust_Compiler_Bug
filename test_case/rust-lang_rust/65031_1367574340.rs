json
$ jq 'sort_by(.total_estimate) | .[-10:] | .[] | { name, total_estimate, instantiation_count }' rustc_query_impl.mono_items.json -c
{"name":"<plumbing::QueryCtxt<'_> as rustc_query_system::query::QueryContext>::start_query::{closure#0}","total_estimate":75240,"instantiation_count":684}
{"name":"std::ptr::mut_ptr::<impl *mut T>::is_null","total_estimate":77119,"instantiation_count":3353}
{"name":"std::ptr::const_ptr::<impl *const T>::is_aligned_to","total_estimate":79212,"instantiation_count":1722}
{"name":"plumbing::force_from_dep_node","total_estimate":84360,"instantiation_count":285}
{"name":"rustc_middle::ty::tls::with_context_opt","total_estimate":84747,"instantiation_count":1599}
{"name":"rustc_query_system::query::plumbing::execute_job","total_estimate":85728,"instantiation_count":228}
{"name":"rustc_query_system::dep_graph::DepGraph::<K>::with_task_impl","total_estimate":89148,"instantiation_count":228}
{"name":"rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory","total_estimate":102372,"instantiation_count":228}
{"name":"hashbrown::raw::RawTable::<T, A>::reserve_rehash","total_estimate":111354,"instantiation_count":277}
{"name":"std::thread::LocalKey::<T>::try_with","total_estimate":186412,"instantiation_count":3214}
