rust
#[derive(Clone)]
struct Ptr<T>(*mut T);

fn foo<T>(ptr: Ptr<T>) {
    ptr.clone()
}

fn main() {}
