
#[inline]
pub fn foo() -> i128 {
    bar().0
}

#[inline]
fn bar() -> (i128, bool) {
    let a = 0;
    let b = 128;
    let shift = 4;
    let ret = baz(a, b, shift);
    // should print "baz(0, 128, 4) -> (8, false)"
    println!("baz({}, {}, {}) -> ({}, {})", a, b, shift, ret.0, ret.1);
    ret
}

#[inline]
fn baz(a: i128, b: i128, shift: u32) -> (i128, bool) {
    if shift == 128 {
        (a, false)
    } else {
        (b >> shift, a >> shift != 0)
    }
}
