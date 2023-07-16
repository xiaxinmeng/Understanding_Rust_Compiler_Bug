rust
trait T {
    fn foo(&self, x: &i32, y: &i32) -> Option<&i32> {
        Some(y)
    }
}
