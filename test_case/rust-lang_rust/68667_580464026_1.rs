rust
pub fn unwrap_combinators(a: Option<i32>, b: i32) -> bool {
    a.map(|t| (t >= b, ()))
     .unwrap_or((false, ()))
     .0
}

pub fn unwrap_manual(a: Option<i32>, b: i32) -> bool {
    match a {
        Some(t) => t >= b,
        None => false
    }
}
