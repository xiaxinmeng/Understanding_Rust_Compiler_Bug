 rust
fn overflowing_neg(self) -> ($t, bool) {
    if self == $t::MIN {
        ($t::MIN, true)
    } else {
        (-self, false)
    }
}
