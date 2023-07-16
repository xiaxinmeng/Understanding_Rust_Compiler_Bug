rust
#![feature(custom_inner_attributes)]
#![feature(cfg_eval)]
#![feature(stmt_expr_attributes)]
fn main() {
    let a = #[cfg_attr(FALSE, rustc_dummy)] {
        #![cfg_eval]
    };
}
