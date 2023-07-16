rust
use std::cmp::Ordering;

pub fn if_else(out: &mut i64, key: i64, b: i64, c: i64) -> bool {
    if key == b {
        return true;
    }
    if key < b {
        *out = b;
    } else {
        *out = c;
    }
    false
}

pub fn contains_match_(out: &mut i64, key: i64, b: i64, c: i64) -> bool {
    match key.cmp(&b) {
        Ordering::Equal => return true,
        Ordering::Less => {
            *out = b;
        },
        Ordering::Greater => {
            *out = c;
        },
    }
    false
}

pub fn if_else_cmp(out: &mut i64, key: i64, b: i64, c: i64) -> bool {
    let ordering = key.cmp(&b);
    if ordering == Ordering::Equal {
        return true;
    }
    if ordering == Ordering::Less {
        *out = b;
    } else {
        *out = c;
    }
    false
}
