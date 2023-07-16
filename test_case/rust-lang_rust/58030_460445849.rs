rust
pub fn saturating_add(self, rhs: Self) -> Self {
    #[cfg(stage0)]
    match self.checked_add(rhs) {
        Some(x) => x,
        None => Self::max_value(),
    }
    #[cfg(not(stage0))]
    {
        intrinsics::saturating_add(self, rhs)
    }
}
