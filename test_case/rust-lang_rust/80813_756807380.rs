rust
// Both `v` and `s` must live for at least the lifetime `'a`.
fn foo<'a>(v: &'a mut Vec<&'a str>, s: &'a str) {
    v.push(s);
}

fn main () {
    let mut v = Vec::new();
    // `s` will be dropped at the end of the scope
    let s = "abc";
    // The compiler sees that `&mut v` and `s` must live for `'a`.
    // Since `s` has a longer lifetime (until the end of the scope),
    // `&mut v`'s lifetime is extended to match that of s.
    foo(&mut v, s);
    // `&mut v` is still live, so taking an immutable reference to `v` is an error
    println!("{:?}", v);
} // The borrows of `s` and `&mut v` end here
