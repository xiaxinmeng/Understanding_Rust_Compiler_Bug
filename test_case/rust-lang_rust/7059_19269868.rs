 rust
fn main() {
    foo();
    bar();

    fn bar() {
        foo();
        println("bar");
    }

    fn foo() {
        println("foo");
    }
}
