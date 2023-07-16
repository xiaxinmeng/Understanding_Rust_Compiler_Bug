rust
struct Ref<'a, T> { ... }
impl Something {
    fn foo(&self) -> Ref<i32>;
}
