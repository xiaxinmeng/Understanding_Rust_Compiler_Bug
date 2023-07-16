rust
pub struct NotCopy;

impl Clone for NotCopy {
    fn clone(&self) -> NotCopy {
        NotCopy
    }
}

pub fn foo(v: &[NotCopy; 50]) -> [NotCopy; 50] {
    v.clone()
}
