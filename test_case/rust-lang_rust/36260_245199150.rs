 rust
fn create_fn() -> Box<Fn()> {
    let text = String::new();

    Box::new(move || { let _ = &text; })
}

fn main() {
    let _ = create_fn();
}
