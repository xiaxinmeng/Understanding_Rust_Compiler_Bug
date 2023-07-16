rust
#![feature(const_generics)]

struct S;
impl S {
    fn test<const A: i32>() -> i32 {
        A
    }
}

fn main() {
    S::test::<{ 16i32 }>();
}
