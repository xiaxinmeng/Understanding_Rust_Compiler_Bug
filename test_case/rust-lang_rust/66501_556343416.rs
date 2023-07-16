rust
fn main() {
    const CONST: &[(); 0] = &[];
    match &[] {
        &[] => {}
        CONST => {}
    }
}
