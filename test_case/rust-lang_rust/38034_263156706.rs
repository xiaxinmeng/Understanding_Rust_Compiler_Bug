rust
self.checked_mul(other).unwrap_or_else(|| if (self ^ other).is_negative() {
    Self::min_value()
} else {
    Self::max_value()
})
