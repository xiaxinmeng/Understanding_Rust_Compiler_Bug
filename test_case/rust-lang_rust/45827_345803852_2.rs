
match **ak {
    AggregateKind::Adt(def, _, substs, _) =>
        tcx.predicates_of(def).instantiate(tcx, substs),
    ...
}
