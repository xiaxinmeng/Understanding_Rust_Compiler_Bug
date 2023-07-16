
fn do_thing<F: FnMut(&usize) -> bool>(mut f: F) -> bool {
    f(&5)
}

fn main() {
    let testfn = |_| true;
    do_thing(testfn);
}
