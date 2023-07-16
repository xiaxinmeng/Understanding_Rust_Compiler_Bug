
fn main() {
    let a = &~1;
    let mut c: ~int;
    match copy *a {
        b => c = b
    };
    log(error, *c);
}
