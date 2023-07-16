rust
impl Step for VarRegister {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        (*end as u8 as usize).checked_sub(*start as u8 as usize)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        u8::try_from(count)
            .ok()
            .and_then(|count| (start as u8).checked_add(count))
            .and_then(|i| Self::try_from(i).ok())
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        u8::try_from(count)
            .ok()
            .and_then(|count| (start as u8).checked_sub(count))
            .and_then(|i| Self::try_from(i).ok())
    }
}
