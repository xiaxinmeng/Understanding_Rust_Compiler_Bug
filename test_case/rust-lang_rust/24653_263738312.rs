rust
fn main() {
}

fn foo<T: ::std::io::Read>(mut v: T) {
    let v = v.chars();
}
