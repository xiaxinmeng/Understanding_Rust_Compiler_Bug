
// Possibly an unstable trait with stable methods
trait IntAdd<Rhs = Self>: Add<Rhs> {
    fn wrapping_add(self) -> Self::Output;
    fn checked_add(self) -> Option<Self::Output>;
    /* other add operations */
}
