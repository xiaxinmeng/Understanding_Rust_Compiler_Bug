 rust
fn foo(it: &mut Iterator<Item = i32>) {
    for x in it {}
//~^ error: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
}

fn main() { }
