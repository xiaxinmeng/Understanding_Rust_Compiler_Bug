 rust
     pub trait AddAssign<RHS>: Add<RHS, Self> {
         #[inline]
         fn add_assign(&mut self, rhs: &RHS) {
             *self = *self + *rhs;
         }
     }
   