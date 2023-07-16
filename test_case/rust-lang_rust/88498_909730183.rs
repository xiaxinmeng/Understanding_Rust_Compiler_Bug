rust
pub enum UnOp {
    Not(Vec<()>),
}

pub fn foo() {
    if let Some(x) = None {
        match x {
            UnOp::Not(_) => {}
        }
    }
}
