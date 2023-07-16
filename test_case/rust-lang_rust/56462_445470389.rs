
query type_of(key: DefId) -> Ty<'tcx> {
	cache { key.is_local() },
	description { "evaluating `finding type of `{:?}`", key },
	fatal_cycle,
	eval_always,
}
