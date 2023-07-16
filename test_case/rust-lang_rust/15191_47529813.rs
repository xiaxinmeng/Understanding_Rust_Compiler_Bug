 rust
pub struct Context<'a> {
    pub view: Option<&'a int>,
}

pub trait RelativeTransform2d<'a> {
    fn trans(&'a self) -> Self;
}

/*
// WORKS WHEN IMPLEMENTING TRAIT DIRECTLY

impl<'a> RelativeTransform2d<'a> for Context<'a> {
    fn trans(&'a self) -> Context<'a> {
        *self
    }
}
*/

pub trait CanTransform<'a, T> {
    fn transform(&'a self) -> T;
}

impl<'a> CanTransform<'a, Context<'a>> for Context<'a> {
    fn transform(&'a self) -> Context<'a> {
        *self
    }
}

impl<'a, T:  CanTransform<'a, T>> RelativeTransform2d<'a> for T {
    fn trans(&'a self) -> T {
        self.transform()
    }
}

fn main() {
    let c = Context { view: None };
    {
        let d = c.trans();
        let _d = d.trans();
    }
}
