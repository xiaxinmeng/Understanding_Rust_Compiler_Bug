rust
#![feature(stmt_expr_attributes)]
fn main() {
    let x = #[allow(dead_code)] 8;
}
