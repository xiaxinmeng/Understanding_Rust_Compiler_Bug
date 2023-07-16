rust
impl Hash for HirId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let data: u64 = unsafe { mem::transmute(*self) };
        state.write_u64(data);
    }
}
