rust
/// Returns true if we can safely ignore borrows of this place.
/// This is true whenever there is no action that the user can do
/// to the place `self` that would invalidate the borrow. This is true
/// for borrows of raw pointer dereferents as well as shared references.
fn ignore_borrow_of(&self) -> bool {
    ...
}
