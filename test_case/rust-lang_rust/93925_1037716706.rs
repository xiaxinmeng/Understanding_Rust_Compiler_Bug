rs
#[must_use]
fn foo() -> i64 {
    4
}

fn main() {
   { foo() };
}
