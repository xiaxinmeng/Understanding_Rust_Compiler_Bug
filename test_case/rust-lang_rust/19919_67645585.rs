 rust
fn foo(x: proc(&int)) {
}

fn bar() {
    foo(proc(_)())
}

fn main() {
}
