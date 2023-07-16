
mod m {
    trait Priv {
        fn f(&self) {}
    }
    impl Priv for super::S {}
    pub trait Pub: Priv {}
}

struct S;
impl m::Pub for S {}

fn g<T: m::Pub>(arg: T) {
    arg.f();
}

fn main() {
    // Compiles on nightly, reports error "source trait is private" on stable and beta
    g(S);
}
