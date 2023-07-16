rust
enum LifetimeUseSet<'tcx> {
    One(&'tcx hir::Lifetime),
    Many,
}
