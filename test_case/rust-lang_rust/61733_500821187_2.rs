
fn main() {
    { 2 } // <- not a trailing expression

    // Stmt( expr!(); ) -> [Stmt(0;)]
    0; // A non-empty statement, not a trailing expression

    // ^^^ Assuming "output(expr!()) ;" is re-parsed and not only "output(expr!())".
    // In the second case ";" would be a separate empty statement, and "0" would be a trailing expression 
}
