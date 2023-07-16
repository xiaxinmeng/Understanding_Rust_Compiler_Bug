rust
#[cfg(stage0)]
pub fn saturating_add(self, rhs: Self) -> Self {
    match self.checked_add(rhs) {
        Some(x) => x,
        None => Self::max_value(),
    }
}

#[cfg(not(stage0))]
pub const fn saturating_add(self, rhs: Self) -> Self {
    intrinsics::saturating_add(self, rhs)
}
