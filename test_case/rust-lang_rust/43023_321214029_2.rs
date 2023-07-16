rust
// ICE on beta, nightly; no ICE on stable.
trait TT {
    #[derive(Debug)]
    type F;
}
