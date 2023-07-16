rust
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Source<'a>(&'a str);

pub fn combine<'b, 'c: 'b>(  // Note the bounding of lifetime `'c` with `'b`
    first: &fn() -> Source<'b>,
    second: &fn() -> Source<'c>,
) -> Option<Source<'b>> {
    let loc1 = first();
    let loc2 = second();
    if loc1 == loc2 {
        Some(loc1)
    } else {
        None
    }
}

fn main() {}
