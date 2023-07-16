
#///
# inner crate
#///
DEBUG 2018-08-20T11:14:02Z: rustc_mir::monomorphize::collector: Collecting roots
DEBUG 2018-08-20T11:14:02Z: rustc_mir::monomorphize::collector: collect_roots: entry_fn = None
#...
DEBUG 2018-08-20T11:14:02Z: rustc_mir::monomorphize::collector: visit_item_use(Instance { def: Item(DefId(0/0:8 ~ testtest[dd34]::foo[0])), substs: [] }, is_direct_call=true)
# ...

#///
# main.rs
#///
DEBUG 2018-08-20T11:14:02Z: rustc_mir::monomorphize::collector: Collecting roots
DEBUG 2018-08-20T11:14:02Z: rustc_mir::monomorphize::collector: collect_roots: entry_fn = Some(DefId(0/0:4 ~ testtest[106a]::main[0]))
#...
