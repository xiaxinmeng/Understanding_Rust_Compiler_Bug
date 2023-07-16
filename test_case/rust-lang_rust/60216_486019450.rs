
struct S<'a, 'b> {
    a: &'a usize,
    b: &'b usize,
}

fn foo() -> S<'_, '_> {
    unimplemented!();
}
fn main() {
}
