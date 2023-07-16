Rust
pub trait Partial<X: ?Sized>: Copy {
}

pub trait Complete {
    type Assoc: Partial<Self>;
}

impl<T> Partial<T> for T::Assoc where
    T: Complete
{
}

// Project(
impl<T> Complete for T {
    type Assoc = T;
}

fn main() {
    let (p, q) = copy::<String>("hello".to_string());
    drop(p);
    println!("{}", q);
}

fn copy<P: Complete>(p: P::Assoc) -> (P::Assoc, P::Assoc) {
    copy2(p)
}

fn copy2<P: Partial<P>>(p: P) -> (P, P) {
    (p, p)
}
