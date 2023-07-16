
query stack during panic:
#0 [mir_built] processing `bar::{{closure}}#0`
#1 [typeck_tables_of] processing `bar`
#2 [typeck_tables_of] processing `bar::{{closure}}#0`
#3 [type_of] processing `bar::{{closure}}#0`
#4 [collect_mod_item_types] collecting item types in top-level module
#5 [analysis] running analysis passes on this crate
