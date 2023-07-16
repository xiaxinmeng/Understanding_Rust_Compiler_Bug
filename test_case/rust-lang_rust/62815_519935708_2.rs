rust
impl<A, B, const NAME_A: &'static str, const NAME_B: &'static str, C> fmt::Debug for C
where C: Closure<Captures = (Capture<NAME_A, A>, Capture<NAME_B, B>)>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (a, b) = self.captures_ref();
        f.debug_struct("closure").field(NAME_A, a).field(NAME_B, b).finish()
    }
}
