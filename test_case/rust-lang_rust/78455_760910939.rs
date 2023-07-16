rust
pub fn filter_map<U: ?Sized, F>(orig: Ref<'b, T>, f: F) -> Result<Ref<'b, U>, Self>
where
    F: FnOnce(&T) -> Option<&U>;
