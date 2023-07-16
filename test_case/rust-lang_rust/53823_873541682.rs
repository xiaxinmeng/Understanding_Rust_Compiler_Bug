rust
pub fn binary_search(slice: &[u64], key: u64) -> Result<usize, usize> {
    binary_search_by(slice, |val| val.cmp(&key) )
}

use std::cmp::Ordering::{self, Greater, Less, Equal};

fn binary_search_by<'a, T, F>(slice: &'a [T], mut f: F) -> Result<usize, usize>
    where F: FnMut(&'a T) -> Ordering
{
    let s = slice;
    let mut size = s.len();
    if size == 0 {
        return Err(0);
    }
    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        // mid is always in [0, size).
        // mid >= 0: by definition
        // mid < size: mid = size / 2 + size / 4 + size / 8 ...
        let cmp = f(unsafe { s.get_unchecked(mid) });
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }
    // base is always in [0, size) because base <= mid.
    let cmp = f(unsafe { s.get_unchecked(base) });
    if cmp == Equal { Ok(base) } else { Err(base + (cmp == Less) as usize) }
}
