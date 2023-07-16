 rust
pub trait FromSlice<'a> { fn new(s : &'a [u8]) -> Self; }

struct Hide<'a> { slice : &'a [u8] }
impl<'a> Hide<'a> { fn new(s: &'a [u8]) -> Hide<'a> { Hide { slice: s } } }
impl<'a> FromSlice<'a> for Hide<'a> {
    fn new(s: &'a [u8]) -> Hide<'a> { Hide::new(s) }
}

#[cfg(hide_works)]
pub fn main () {
    let bar : Hide = {
        let foo = ~[1, 2, 3, 4, 5];
        FromSlice::new(foo.as_slice())
    }; // somehow sidesteps typecheck

    let _x : ~[u8] = ~[9,9,9,9,9,9,9,9,9,9,9,9];

    println!("bar.slice = {}, should reject (was [1, 2, 3, 4, 5])", bar.slice);
}

#[cfg(hide_fails)]
pub fn main () {
    let bar : Hide = {
        let foo = ~[1, 2, 3, 4, 5];
        Hide::new(foo.as_slice())
    }; // does not typecheck: `foo` does not live long enough

    let _x : ~[u8] = ~[9,9,9,9,9,9,9,9,9,9,9,9];

    println!("bar.slice = {}, should reject (was [1, 2, 3, 4, 5])", bar.slice);
}
