rust
#![crate_type="lib"]

pub fn map_on_the_elems(v: &Vec<u64>, b: fn(u64)) {
    // Uncomment this to remove the bounds check.
    //let v = &v[..];
    for i in 0 .. v.len() {
        b(v[i]);
    }
}
