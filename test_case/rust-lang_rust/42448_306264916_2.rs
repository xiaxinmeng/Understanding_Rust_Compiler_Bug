rust
macro m($i:ident) {
    struct $i;
}
fn main() {
    m!(CreatedStruct);
    CreatedStruct; // resolves
}
