 rust
#[crate_type="lib"];

#[inline(never)]
pub fn func(foo: Option<int>) {
    println!("{:?}", foo);
}

#[inline(never)]
pub fn func2(foo: int) {
    println!("{:?}", foo);
}

pub fn bar(foo_opt: Option<int>) {
    func(foo_opt);
    match foo_opt {
        Some(foo) => func2(foo),
        _ => {}
    }
}
