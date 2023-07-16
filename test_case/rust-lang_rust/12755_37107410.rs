
pub struct Foo<'a> {
    priv a: &'a mut int,
    priv b: int,
    priv take_a: bool
}

impl<'a> Foo<'a> {
    fn take(&'a mut self) -> &'a mut int {
        if self.take_a {
            &mut *self.a
        } else {
            &mut self.b
        }
    }
}

fn main() { }
