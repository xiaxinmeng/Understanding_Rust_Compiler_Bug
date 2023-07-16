rust
struct A<T> {
    a: T,
}

struct DropStruct;

impl Drop for A<DropStruct> {
    fn drop(&mut self) {}
}

fn main() {
    // Comment the next line to get the real error
    let _ = A { a: 0 };
}
