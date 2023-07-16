rust
pub fn overflowing_add(mut self, other: Uint) -> (Uint, bool) {
    let mut carry = false;
    let mut carry_out;
    for i in 0..Self::LIMBS {
        (self.0[i], carry_out) = self.0[i].carrying_add(other.0[i], carry);
        carry = carry_out;
    }
    (self, carry)
}
