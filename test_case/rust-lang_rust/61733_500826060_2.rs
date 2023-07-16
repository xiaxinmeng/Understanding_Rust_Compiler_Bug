
fn main() {
    { 2 } // <- not a trailing expression

    // Stmt( stmt_expr!(); ) -> [Stmt(0;), Stmt(1;)]
    0; 1; // A non-empty statement not a trailing expression, followed by a non-empty statement not a trailing expression
}
