rust
fn weird_situation<F: FnOnce() -> str>() {
    println!("if this is reachable, there’s a function type with unsized Output type `str`");
}

fn main() {
    weird_situation::<fn() -> str>()
}
