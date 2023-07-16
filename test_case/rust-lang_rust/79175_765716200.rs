rust
fn foo() {}
fn bar() {}

fn do_stuff(x: bool) {
    if x {
        foo()
    } else {
        bar()
    }
}

fn main() {
    do_stuff(false);
}
