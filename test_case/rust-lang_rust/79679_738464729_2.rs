rust
/// `ManuallyDrop` robustly prevents double-free because we disable `v`'s destructor
/// before doing anything else. `mem::forget()` doesn't allow this because it consumes its
