rust
fn is_static<F: 'static>(f: F) -> F { f }

fn foo() {
    let x = 42;
    is_static(&|| { let x = x; x });
}
