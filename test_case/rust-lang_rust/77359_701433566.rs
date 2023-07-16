rust
fn main() {
    println!("{:?}", f(Some(A::Y)));
}

#[derive(Eq, Debug, PartialEq)]
pub enum A {
    X(String),
    Y,
}

#[derive(Eq, Debug, PartialEq)]
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
