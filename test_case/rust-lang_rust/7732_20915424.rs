 rust
struct A { a: Option<@mut A> }

fn main() {
    let a = @mut A { a: None };
    a.a = Some(a);
}

