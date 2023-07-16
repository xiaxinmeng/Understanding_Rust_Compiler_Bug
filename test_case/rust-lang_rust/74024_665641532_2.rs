rust
use std::cmp::Ord;
use std::cmp::Ordering::{self, Equal, Greater, Less};

pub fn std_binary_search<T>(s: &[T], x: &T) -> Result<usize, usize>
where
    T: Ord,
{
    std_binary_search_by(s, |p| p.cmp(x))
}

pub fn std_binary_search_by<'a, T, F>(s: &'a [T], mut f: F) -> Result<usize, usize>
where
    F: FnMut(&'a T) -> Ordering,
{
    let mut size = s.len();
    if size == 0 {
        return Err(0);
    }
    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        // mid is always in [0, size), that means mid is >= 0 and < size.
        // mid >= 0: by definition
        // mid < size: mid = size / 2 + size / 4 + size / 8 ...
        let cmp = f(unsafe { s.get_unchecked(mid) });
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }
    // base is always in [0, size) because base <= mid.
    let cmp = f(unsafe { s.get_unchecked(base) });
    if cmp == Equal {
        Ok(base)
    } else {
        Err(base + (cmp == Less) as usize)
    }
}

pub fn stdnew_binary_search<T>(s: &[T], x: &T) -> Result<usize, usize>
where
    T: Ord,
{
    std_binary_search_by(s, |p| p.cmp(x))
}

pub fn stdnew_binary_search_by<'a, T, F>(s: &'a [T], mut f: F) -> Result<usize, usize>
where
    F: FnMut(&'a T) -> Ordering,
{
    let mut size = s.len();
    if size == 0 {
        return Err(0);
    }
    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        // mid is always in [0, size), that means mid is >= 0 and < size.
        // mid >= 0: by definition
        // mid < size: mid = size / 2 + size / 4 + size / 8 ...
        let cmp = f(unsafe { s.get_unchecked(mid) });
        if cmp == Equal {
            return Ok(mid);
        } else if cmp == Less {
            base = mid
        }
        size -= half;
    }
    // base is always in [0, size) because base <= mid.
    let cmp = f(unsafe { s.get_unchecked(base) });
    if cmp == Equal {
        Ok(base)
    } else {
        Err(base + (cmp == Less) as usize)
    }
}
