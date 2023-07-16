
> rg -w walk_tys src
src/librustc/ty/mod.rs
1350:    pub fn walk_tys(&self) -> IntoIter<Ty<'tcx>> {

src/librustc/ty/sty.rs
1855:    /// types reachable from this type via `walk_tys`). This ignores late-bound

src/librustdoc/clean/auto_trait.rs
408:        pred.walk_tys()
