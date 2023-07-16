rust
trait Trait { }
impl<T> Trait for T { }

impl<'a> PartialEq for Trait + 'a {
    fn eq(&self, other: &Self) -> bool { false }
}

fn main() {
    let x = Box::new(1) as Box<Trait>;
    let y = Box::new(1) as Box<Trait>;
    let equal = x == y;
}
