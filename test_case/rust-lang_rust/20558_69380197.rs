 rust
fn foo<F: FnMut(&[u8])>(mut f: F) {
    f(b"hello ");
    f(b"world!");
}

fn main() {
    let mut v = Vec::new();
    // error: unable to infer enough type information about `closure[...]`; type annotations required
    foo(|buf| v.extend(buf.iter().cloned()));
    println!("{:?}", v);
}
