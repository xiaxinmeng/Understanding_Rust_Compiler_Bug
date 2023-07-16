rust
pub enum AsyncGeneratorKind {
    /// An explicit `async` block written by the user.
    Block,

    /// An explicit `async` block written by the user.
    Closure,

    /// The `async` block generated as the body of an async function.
    Fn,
}
