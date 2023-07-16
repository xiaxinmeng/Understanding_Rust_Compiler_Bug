
fn main() {
    { 2 } // not a trailing expression

    // Stmt( stmt_expr!() ) -> [Stmt(0;), Stmt(1)]
    0; 1 // trailing expression
}
