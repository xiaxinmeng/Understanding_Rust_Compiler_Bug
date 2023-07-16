rust
trait Eq {
    #[requires(neq)]
    fn eq(&self, other: &Self) -> bool { !self.neq(other) }

    #[requires(eq)]
    fn neq(&self, other: &Self) -> bool { !self.eq(other) }
}
