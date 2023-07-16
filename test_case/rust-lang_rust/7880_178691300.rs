
fn returns_option() -> Option<i32> {
    Some(5)
}

fn do_things(_x: &Option<i32>) {
}

fn main() {
    let foo = returns_option();
    do_things(&foo);
    match foo {
            Some(ref a) => { /* use a */ }
                None => {}
    }
}
