
fn bar(f: &fn()) {}

fn foo(f: &fn()) {
    bar(||f());
    bar(||f());
}

fn baz(fs: &[&fn()]) {
    for f in fs.iter() {
        bar(||(*f)());
    }
}
