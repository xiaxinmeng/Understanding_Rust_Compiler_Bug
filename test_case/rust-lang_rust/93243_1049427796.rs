rust
trait Iterator {
	fn into_random_access(self) -> <Self as IterToRandom>::Random;
		where Self: IterToRandom;
}

trait IterToRandom {
	type Random: RandomAccess;
	fn into(self) -> Self::Random;
}

unsafe trait RandomAccess {
	type Item;
	fn size(&self) -> usize;
	// If false, then callers which generally want to
	// preserve side-effects can skip get_at calls.
	const MAY_HAVE_SIDE_EFFECTS: bool;
	// Caller guarantees to only call this with a given `idx` once
	// on a given instance of `self`. (Caller may call multiple times if `Self: Clone`).
	// Calls must be such that `0 <= idx < self.size()`.
	unsafe fn get_at(&mut self, idx: usize) -> Self::Item;
};
