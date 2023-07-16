 rust
trait Foo<'self> {
    fn a(&self) -> &'self str;
    fn b(&self) -> &'self str;
}

impl<'self> Foo<'self> for &'self str {
    fn a(&self) -> &'self str {
        *self
    }
    fn b(&self) -> &'self str {
        *self
    }
}

fn main () {
    let c = "defg";
    c.a();
    c.b();
    c.a().b();
}
