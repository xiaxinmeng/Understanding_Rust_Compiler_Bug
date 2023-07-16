rust
 #![feature(stmt_expr_attributes)] 
 #![crate_type="rlib"]

pub fn main() {
    let x = #[inline(always)] || {};
    x();
}
