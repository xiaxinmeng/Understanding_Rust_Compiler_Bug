Rust
pub struct LocalTableInContext<'a, V> {
    parent_def_id: Option<DefId>,
    data: &'a LocalItemMap<V>
}
