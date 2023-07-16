rust
use std::cmp::Ordering::{self, *};

fn binary_search_by<T, F>(this: &[T], mut f: F) -> Result<usize, usize>
where
    F: FnMut(&T) -> Ordering
{
    let mut base = 0usize;
    let mut s = this;

    loop {
        let (head, mid, tail) = match s.split_at(s.len() >> 1) {
            (_, []) => return Err(base),
            (head, [mid, tail @ ..]) => (head, mid, tail),
        };
        
        let cmp = f(mid);
        if cmp == Less {
            base += head.len() + 1;
            s = tail;
        } else if cmp == Greater {
            s = head;
        } else {
            return Ok(base + head.len());
        }
    }
}

pub fn foo(data: &[u32], x: u32) -> Result<usize, usize> {
    binary_search_by(data, |y| x.cmp(y))
}
