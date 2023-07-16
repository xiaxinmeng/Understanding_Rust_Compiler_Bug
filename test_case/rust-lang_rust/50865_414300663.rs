
#///
# inner crate
#///
DEBUG 2018-08-20T10:26:39Z: rustc_mir::monomorphize::collector: Collecting roots
DEBUG 2018-08-20T10:26:39Z: rustc_mir::monomorphize::collector: collect_roots: entry_fn = None
# ...

#///
# main.rs
#///
DEBUG 2018-08-20T10:26:39Z: rustc_mir::monomorphize::collector: Collecting roots
DEBUG 2018-08-20T10:26:39Z: rustc_mir::monomorphize::collector: collect_roots: entry_fn = Some(DefId(0/0:4 ~ testtest[106a]::main[0]))
# ...
DEBUG 2018-08-20T10:26:40Z: rustc_mir::monomorphize::collector: visit_item_use(Instance { def: Item(DefId(9/0:8 ~ 
testtest[dd34]::foo[0])), substs: [] }, is_direct_call=true)

error: internal compiler error
: librustc_mir\monomorphize\collector.rs:767: Cannot create local mono-item for DefId(9/0:8 ~ testtest[dd34]::foo[0])
