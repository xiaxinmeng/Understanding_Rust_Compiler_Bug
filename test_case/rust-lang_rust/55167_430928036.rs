rust
enum MissingCtors<'tcx> {
	Empty,
	Nonempty,
	NonemptyWithCtors(Vec<Constructor<'tcx>>),
}
