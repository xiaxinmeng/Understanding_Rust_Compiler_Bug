 Rust
trait Mirror {
    type Image;
}

impl<T> Mirror for T { type Image = T; }

trait Foo {
    fn recurse(&self);
}

impl<T> Foo for T {
    #[allow(unconditional_recursion)]
    fn recurse(&self) { 
        (self, self).recurse();
    }
}

fn main() {
    ().recurse();
}
