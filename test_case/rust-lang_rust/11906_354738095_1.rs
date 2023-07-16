rust
fn bar() {
}

fn foo(a: i64, (b, c): (i64, i64)) {
    bar();
}

fn main() {
    foo(1, (2, 3));
}
