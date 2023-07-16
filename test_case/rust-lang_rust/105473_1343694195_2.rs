rs
fn foo() {
    bar(17);
    foo();
}

fn bar(x: i32) {
    if x > 12 {
        std::process::exit(0);
    }
}
