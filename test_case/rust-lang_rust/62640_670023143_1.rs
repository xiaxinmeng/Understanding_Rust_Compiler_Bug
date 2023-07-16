
fn main() {
    let mut a = 1;
    let b = &mut a;
    let foo = |state| {
        Box::new(state);
    };

    foo(b);
    foo(b);
}
