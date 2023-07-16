rust
#![crate_type="lib"]

pub fn map_on_the_elems(v: &Vec<u64>, b: fn(u64)) {
    let len = v.len();
    let mut i = 0;
    while len > i {
        b(index(&v[..], i));
        i += 1;
    }
}

#[inline(always)]
fn index(v: &[u64], idx: usize) -> u64 {
    v[idx]
}
