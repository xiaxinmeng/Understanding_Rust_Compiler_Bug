 rust
fn foo<'a>(x: &'a int) -> &'a int {
    x
}

static FOO: int = 5;

fn main() {
    foo(&FOO);
    let x = 10;
    foo(&x);
}
