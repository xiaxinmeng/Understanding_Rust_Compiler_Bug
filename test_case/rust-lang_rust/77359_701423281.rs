rust
fn main() {
    f(None);
}

pub enum A {
    X,
    Y,
}

pub enum B {
    B(Option<A>),
}

pub fn f(a: Option<A>) -> B {
    if let Some(x) = a {
        let y = x;
        return B::B(Some(y));
    }
    B::B(None)
}
