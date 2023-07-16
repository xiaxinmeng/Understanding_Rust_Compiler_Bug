
fn main() {
    { 2 } // not a trailing expression

    // Stmt( stmt!() ) -> [Stmt(0;)]
    0; // not a trailing expression
}
