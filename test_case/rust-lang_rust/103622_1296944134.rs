rust
struct Type;

pub trait Trait {
    fn function(&mut self) where Self: Sized;
}

impl Trait for Type {
    fn function(&mut self) {}
}

fn main() {
    (&mut Type as &mut dyn Trait).function();
}
