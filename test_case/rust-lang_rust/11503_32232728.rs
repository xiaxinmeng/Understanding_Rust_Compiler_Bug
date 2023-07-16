
Self: Mul<T, T>
fn square(&self) -> Self {self * self}
fn square_in_place(&mut self) -> Self {*self = self * self;}

Self: One+Mul<T, T>>, U: Zero+Shr<uint, U>
fn pow(&self, exp: &U) -> T {
    if(exp == Zero::zero()) {return One::one();}
    if(exp == One::one() {return self.clone();}
    // here exp >= 2, so we can start forming these and looking for a set bit after bit 0
    let mut i: U = exp >> 1;
    let mut v: T = self.square();
    let mut r: T;

    // find least significant bit set starting from bit 1, write its power in res, possibly multiplied by base if bit 0 is set
    // return if no other bits set
    // otherwise point i and v to the bit after that
    loop {
        if(i.is_odd()) {
            break;
        }
        i >>= 1;
        v.square_in_place();
    }
    i >>= 1;
    if(i.is_zero()) {return if exp.is_odd() {self * v} else {v};}
    if exp.is_odd() {
        r = self * v;
        v.square_in_place();
    } else {
        let v2 = v.square();
        r = v;
        v = v2;
    }

    // multiply in all other bits
    loop {
        if(i.is_odd()) {
            r *= v;
            i >>= 1;
            if(i.is_zero()) {return res;}
        } else {
            i >>= 1;
        }
        v.square_in_place();
    }
}
}
