rust
fn coupled_wilds_rhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
    let ((y, _z),) = ((s, _x),): (PairCoupledTypes<_>,);
    y //~ ERROR unsatisfied lifetime constraints
}
