rust
fn main() {
    let x = vec![()];
    x.into_iter().flat_map(|_| Some(1)).next();
    // ^ should probably suggest `find_map`.
}
