rust
fn willy<'w>(p: &'w str) -> &'w str {
    let free_dumb = |x: &str| -> &str { p };
    let hello = format!("Hello");
    free_dumb(&hello)
}

fn main() {
    let world = format!("World");
    willy(&world);
}
