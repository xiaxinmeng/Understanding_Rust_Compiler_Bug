rust
// this trips this deprecation lint
fn item() {
    #[derive(Foo)]
    struct Blah;
}
