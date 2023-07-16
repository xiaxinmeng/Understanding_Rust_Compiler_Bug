rust
impl Default for Thing {
    fn default() -> Thing {
        Thing { one: One::default() }
    }
}
