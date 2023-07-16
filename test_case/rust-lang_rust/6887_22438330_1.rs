 rust
fn foo(x: Option<Option<int>>) -> int {
    match x {
        Some(Some(n)) => n,
        Some(None) => 3
    }
}

fn main() {
    println(fmt!("%d", foo(Some(Some(4)))));
}
