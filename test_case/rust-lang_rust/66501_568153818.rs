rust
const CONST: &[(); 1] = &[()];
fn main() {
    match &[()] {
        ::CONST => {}
    }
}
