 rust
impl<A, B, C, F: |A| -> B, G: |B| -> C, FG: |A| -> C> Mul<G, FG> for F {
    fn mul(&self, g: &G) -> FG {
        |a| g(self(a))
    }
}
