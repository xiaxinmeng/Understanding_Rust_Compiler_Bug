rust
pub fn hash(bind_group: *const BindGroup, state: &mut impl std::hash::Hasher) {
    bind_group.hash(state)
}
