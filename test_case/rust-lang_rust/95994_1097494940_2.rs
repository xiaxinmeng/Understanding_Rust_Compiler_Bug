rust
#[derive(Clone, Copy, PartialEq)]
enum IndentStyle {
    /// Vertically aligned under whatever column this block begins at.
    ///
    ///     fn demo(arg1: usize,
    ///             arg2: usize);
    Visual,
}
