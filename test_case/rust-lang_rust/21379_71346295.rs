 rust
fn foo(it: &mut Iterator<Item = i32>) {
    for x in *it {}
}

fn main() { }
