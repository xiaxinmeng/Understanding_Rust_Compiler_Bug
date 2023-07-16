rust
pub trait Bar { fn m(&self); }

pub fn foo<T>() {
    struct S<T>;
    impl Bar for S<T> {
        fn m(&self) { println!("Hello World"); }
    }
    let s = S::<T>;
    s.m();
}

pub fn main() {
}
