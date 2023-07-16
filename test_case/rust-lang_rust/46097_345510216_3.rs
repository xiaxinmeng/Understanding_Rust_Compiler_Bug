rust
impl BorrowckMode {
    /// Should the AST-based borrow checker execute at all?
    pub fn use_ast(self) -> bool {
        match self {
            BorrowckMode::Ast => true,
            BorrowckMode::Compare => true,
            BorrowckMode::Mir => false,
        }
    }
    ...
}
