 rust
pub fn map_ref<'b, T: ?Sized, U: ?Sized, F>(orig: Ref<'b, T>, f: F) -> Ref<'b, U>
where F: FnOnce(&T) -> &U
