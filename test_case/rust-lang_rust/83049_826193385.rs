rust
mod foo {
    fn error() {
        super::bar::baz()
    }
}

mod bar {
    fn baz() {}
}
