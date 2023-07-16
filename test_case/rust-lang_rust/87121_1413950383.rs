rust
trait Match {
    type Target: ?Sized;
    fn do_match(&self) -> &Self::Target;
}
trait MatchMut: Match {
    fn do_match_mut(&mut self) -> &mut Self::Target;
}
