rs
pub enum Ordering {
    Less, Equal, Greater
}

pub fn c(ord: Ordering) -> i32 {
        match ord {
        Ordering::Less => 0,
        Ordering::Equal => 1,
        Ordering::Greater => 2,
    }
}

pub fn d(ord: Ordering) -> i32 {
        match ord {
        Ordering::Less => 0,
        Ordering::Equal => 1,
        _ => 2,
    }
}
