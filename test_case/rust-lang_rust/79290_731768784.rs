rust
fn main() {
    let fct = bar; // create a reference to bar
    foo(fct); // direct call to foo
}

fn foo<Fct: Fn()>(fct: Fct) {
    fct(); // indirect call to bar, indirection set in `main`
}

fn bar() {}
