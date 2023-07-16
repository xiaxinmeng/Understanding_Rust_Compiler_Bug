 rust
fn call(f: fn()) {
    f()
}

fn foo() {
    println!("foo");
}

fn main() {
    call(foo);
}
