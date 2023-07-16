rust
/// `(self << rhs) | carry`
fn carrying_shl(self, rhs: u32, carry: Self) -> (Self, Self); 

/// `(self >> rhs) | carry`
fn borrowing_shr(self, rhs: u32, carry: Self) -> (Self, Self);

/// `self << rhs`
fn widening_shl(self, rhs: u32) -> (Self, Self);

/// `self >> rhs`
fn widening_shr(self, rhs: u32) -> (Self, Self);
