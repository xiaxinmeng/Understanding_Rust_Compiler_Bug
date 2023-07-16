rust
// Invariant: the integer is 0, except behind shared *unpinned* references it can be whatever.
// There is no `&self` operation so we do not care about those values.
struct Nasty(i32);

impl Nasty {
    pub fn new() -> Self {
        Nasty(0)
    }
    
    pub fn new_shared() -> &'static Self {
        // We seem to violate the invariant here, but there is no operation
        // on `&self` so this is not a problem.
        &Nasty(1)
    }
    
    pub fn check(self: Pin<&Self>) {
        // It would be sound to cause UB for non-0 values here.
        // IOW, this assertion can never fail (for safe callers).
        assert!(self.0 == 0);
    }
}
