rust
trait Sequencer {
    type Wrap<A>;
    fn chain<A, B, F>(Self::Wrap<A>, F) -> Self::Wrap<B>
        where F: FnOnce(A) -> Self::Wrap<B>;
    fn wrap<A>(A) -> Self::Wrap<A>;
}
