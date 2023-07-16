
fn main() {
    let mut a = 1;
    let b = &mut a;
    let foo = |state: &mut i32| {
        Box::new(state);
    };

    foo(b);
    foo(b);
}
