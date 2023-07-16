rust
#![feature(specialization)]

#[macro_use]
extern crate ast_node;
extern crate swc_common;

#[derive(Fold)]
pub struct Expr {
    pub super_class: Option<Box<Expr>>,
}
