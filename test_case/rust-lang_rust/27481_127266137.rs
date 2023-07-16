 rust
impl<C: ChangeSet> ChangeSet for TrackedMapChangesInternal<C, C::Value /* need to know this first */> {
    /* you never get here, because C::Value has to be figured out first, therefore C::Value is never figured out */
    type Value = TrackedMap<C>;
}
