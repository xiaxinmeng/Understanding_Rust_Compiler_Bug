
struct S;

mod m {
    fn f() {
        impl ::S {
            pub fn s(&self) {}
        }
    }
}

fn main() {
    S.s() // Privacy error, unless `fn s` is pub
}
