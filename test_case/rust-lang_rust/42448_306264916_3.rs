rust
macro m() {
    struct #CreatedStruct; // This is exempt from hygiene, so is usable outside the macro.
    let #created_local = 0; // Same with this local.
}
fn main() {
    m!();
    CreatedStruct; // resolves
    created_local; // resolves
}
