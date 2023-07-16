rust
use std::sync::Arc;

pub trait Symbol: 'static {
    fn diff_dyn(&self) -> Arc<dyn Symbol> {
        unimplemented!()
    }
}
impl Symbol for Arc<dyn Symbol> {}

#[derive(Clone)]
pub struct Variable;
impl Symbol for Variable {}

pub struct DynExpr(pub Arc<dyn Symbol>);

pub struct BinarySym<Op, Sym1, Sym2> {
    _op: Op,
    sym1: Sym1,
    sym2: Sym2,
}
impl<Op, Sym1, Sym2> BinarySym<Op, Sym1, Sym2> {
    pub fn new(_sym1: Sym1, _sym2: Sym2) -> Self {
        unimplemented!()
    }
}
impl<Op, Sym1, Sym2> Clone for BinarySym<Op, Sym1, Sym2> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

pub struct MulOp;
pub type MulSym<Sym1, Sym2> = BinarySym<MulOp, Sym1, Sym2>;
impl<Sym1, Sym2> Symbol for MulSym<Sym1, Sym2>
where
    Sym1: Symbol,
    Sym2: Symbol,
{
}

pub struct DivOp;
pub type DivSym<Sym1, Sym2> = BinarySym<DivOp, Sym1, Sym2>;
impl<Sym1, Sym2> Symbol for DivSym<Sym1, Sym2>
where
    Sym1: Symbol + Clone,
    Sym2: Symbol + Clone,
{
    fn diff_dyn(&self) -> Arc<dyn Symbol> {
        Arc::new(DivSym::new(
            MulSym::new(self.sym1.clone(), self.sym2.clone().diff_dyn()),
            MulSym::new(self.sym2.clone(), self.sym2.clone()),
        ))
    }
}

fn main() {
    let _div = DynExpr(Arc::new(DivSym::new(Variable, Variable)));
}
