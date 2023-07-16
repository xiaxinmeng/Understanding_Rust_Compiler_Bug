 Rust
#[doc = "Iterate over the range [`begin`..`end`)"]
fn range(begin: i16, end: i16, it: fn(i16)) {
    let i = begin;
    if begin < end {
        while i < end { it(i); i += 1i16; }
    } else {
        while i > end { it(i); i -= 1i16; }
    }
}
