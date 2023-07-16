rust
fn foo(ref x: i32) {}

fn bar() {
    let mut x = 0;
    move || {
        x = 1;
    };
}
