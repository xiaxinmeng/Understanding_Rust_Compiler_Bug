rust
fn main() {
    { () } // trailing expression

    #[empty]
    0; // not trailing expression
}
fn main() {
    { () } // trailing expression

    #[empty]
    0 // not trailing expression
}
fn main() {
    { () } // not trailing expression

    #[expr]
    0; // trailing expression (the attribute transforms the whole statement, including semicolon)
        // so it's equivalent to `expr!( 0 ; )`
}
fn main() {
    { () } // not trailing expression

    #[expr]
    0 // trailing expression
}
fn main() -> u8 {
    { () } // not trailing expression

    #[stmt]
    0; // not trailing expression
}
fn main() -> () {
    { () } // not trailing expression

    #[stmt]
    0 // error: macro expansion ignores token `;` and any following
}
fn main() -> () {
    { () } // not trailing expression

    #[stmt_expr]
    0; // trailing expression (the attribute transforms the whole statement, including semicolon)
        // so it's equivalent to `stmt_expr!( 0 ; )`
}
fn main() -> () {
    { () } // not trailing expression

    #[stmt_expr]
    0 // error: macro expansion ignores token `;` and any following
}
