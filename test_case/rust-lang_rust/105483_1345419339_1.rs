rust
struct Element<'a>(&'a ());
struct Wrapper<'a>(Element<'a>);

trait Extract {
    fn extract(&self) -> &'_ Element;
}
impl<'a> Extract for Wrapper<'a> {
    fn extract(&self) -> &'a Element {
        &self.0
    }
}
