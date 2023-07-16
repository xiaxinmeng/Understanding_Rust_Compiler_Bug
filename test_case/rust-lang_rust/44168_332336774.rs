rust
pub trait HasLength {
    const LENGTH: usize;
}

fn foo<T: Copy + HasLength>(x: T) -> [T; <T as HasLength>::LENGTH] { [x; <T as HasLength>::LENGTH] }
