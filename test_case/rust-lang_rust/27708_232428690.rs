 rust
/// Get a raw pointer to the data contained in this `RefCell`.
///
/// The returned pointer will point directly at the data contained within, and
/// this method will make no modifications to the borrow flag of this `RefCell`
/// as well.
///
/// Note that it is not safe to use this pointer if the `RefCell` is borrowed
/// mutably at this time, but if this `RefCell` is not mutably borrowed it is
/// safe to use the returned pointer as `&T` at least.
pub fn get_ptr(&self) -> *mut T { /* ... */ }
