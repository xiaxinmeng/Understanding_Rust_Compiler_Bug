rust
#![recursion_limit = "48"]

fn f<B>(x: Vec<[[[B; 1]; 1]; 1]>) {
    is_eq::<_, B>(x);
}

fn is_eq<T: PartialEq<B>, B>(_: T) {}

fn main() {}
