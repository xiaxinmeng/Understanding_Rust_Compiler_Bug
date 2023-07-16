rust
trait Mine {}

impl Mine for i32 {}

impl PartialEq for dyn Mine {
    fn eq(&self, _other: &dyn Mine) -> bool {
        true
    }
}

fn main() {
    let b1: Box<dyn Mine> = Box::new(1);
    let b2: Box<dyn Mine> = Box::new(1);
    if b1 == b2 {}
    if b1 == b2 {}
}
