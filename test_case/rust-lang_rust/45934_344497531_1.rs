rust
fn f() {
    mod foo {
        pub fn g();
    }
    use ??::foo::g; // How to import `g`?
}
