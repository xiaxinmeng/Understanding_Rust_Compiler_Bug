 rust
fn main() {
    {
        extern fn foo() { }
    }

    {
        extern fn foo(_bar: usize) { }
    }
}
