
fn call_once(action: proc() -> T) -> T {
    action()
}

fn consume_owned(_text: ~str) {}
fn used_borrowed(_text: &int) {}

fn lets_try(foo: &int, bar: ~str, baz: &mut_int) {
    mut int qux = 0;
    do call_once {
        used_borrowed(foo);
        consume_owned(bar);
        *baz += 1;
        qux += 1;
    }
}
