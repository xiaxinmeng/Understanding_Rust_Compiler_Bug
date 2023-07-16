rust
struct Foo<T> { t: Option<T> }

unsafe impl Send for Foo<u32> { }

fn foo() -> impl Send {
    Foo { t: None }
}

fn main() { }
