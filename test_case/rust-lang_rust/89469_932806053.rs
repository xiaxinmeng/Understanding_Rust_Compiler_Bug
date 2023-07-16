rust
#[allow(non_snake_case)]
struct Entry {
    A: u16,
    a: u16
}

fn foo() -> Entry {todo!()}

pub fn f() {
    let Entry { A, a } = foo();
    let _ = (A, a);
}
