
trait AddAssign<RHS> {
    fn add_assign(&mut self, rhs: &RHS); //method
    fn add_assign<???>(&mut self, rhs: &RHS) { *self = *self + *rhs; } //default
}
