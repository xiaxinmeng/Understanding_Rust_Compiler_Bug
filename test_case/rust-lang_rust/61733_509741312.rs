rust
fn main() {
    // fn main() {}
    empty!()
}
fn main() {
    // fn main() { 0; 1 }, trailing
    stmt_expr!()
}
fn main() {
    // fn main() { 0; }, non-trailing
    #[stmt]
    0
}
fn main() {
    // fn main() { 0; 1 }, trailing
    #[stmt_expr]
    0
}
