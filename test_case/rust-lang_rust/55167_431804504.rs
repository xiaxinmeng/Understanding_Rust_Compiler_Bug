rust
/// A request for missing constructor data in terms of either:
///	(a) Simply whether there are any missing constructors.
///	(b) Exactly which constructors are missing.
/// This is to improve performance in cases where the full constructor information is unnecessary.
enum MissingCtorsInfo {
	/// Corresponds to `MissingCtors::Empty` and `MissingCtors::Nonempty`.
	Emptiness,
	/// Corresponds to `MissingCtors::NonemptyWithCtors`.
	Ctors,
}
