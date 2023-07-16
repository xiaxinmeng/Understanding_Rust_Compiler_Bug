 rust
use std::ops::BitAndAssign;

#[derive(Debug, PartialEq)]
struct BooleanVector {
    value: [bool; 4],
}

impl BitAndAssign for BooleanVector {
    fn bitand_assign(&mut self, rhs: Self) {
        for (x, y) in self.value.iter_mut().zip(&rhs.value) {
            *x &= *y;
        }
    }
}

fn main() {
    let mut bv = BooleanVector { value: [true, true, false, false] };
    bv &= BooleanVector { value: [true, false, true, false] };
    let expected = BooleanVector { value: [true, false, false, false] };
    assert_eq!(bv, expected);
}
