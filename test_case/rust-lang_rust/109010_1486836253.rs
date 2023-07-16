rust
trait HealthCheck {
    // idk about `'anon`, need to provide some way to talk about the anonymous lifetimes
    // in the return type of `check`.
    type CheckFut<'anon>;
    // The attribute name is a placeholder.
    #[defined_by_returned_future(CheckFut)]
    async fn check(&mut self, server: Server);
}
