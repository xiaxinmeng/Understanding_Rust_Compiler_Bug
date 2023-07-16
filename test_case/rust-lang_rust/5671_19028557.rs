
fn main() { }
struct Foo { x: int }
impl Foo {
    fn bleh(&self) -> &'self int {
        return &self.x
    }
}
