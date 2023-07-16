rust
#[inline(never)]
fn do_something(x: u32) -> bool {
    104729 % x == 0
}

pub fn check(x: &[u32]) -> Option<usize> {
    x.iter().position(|y| do_something(*y))
}
