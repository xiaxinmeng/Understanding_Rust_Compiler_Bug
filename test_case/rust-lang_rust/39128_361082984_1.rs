rust
impl PartialEq for S {
    fn eq(&self, rhs: &S) -> bool {
        match *rhs {
            S(ref rhs) => match *self {
                S(ref lhs) => true && (*lhs) == (*rhs)
            }
        }
    }
}
