 rust
fn f<Generic: AsRef<Concrete>>(arg: Generic) {
    _f(arg.as_ref())
}

fn _f(arg: &Concrete) {
    // ...
}
