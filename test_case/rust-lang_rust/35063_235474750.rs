 rust
mod foo {
    pub fn bar() {}
    mod bar {}
}
fn main() {
    use foo::bar; // Before this PR, the inaccessible module `foo::bar` was imported,
    mod bar {} // so this was a duplicate error.
}
