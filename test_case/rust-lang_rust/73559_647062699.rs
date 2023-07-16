rust
struct A;

impl B for A {
    type C = ();
}

pub trait B {
    type C;
}

type D = <A as B>::C;

trait E: From<D> {
    fn e(self) -> D;
}

trait F {
    type O: E;
    
    fn f(self) -> Self::O;
}

fn h<G: F>(g: G) {
    g.f().e();
}
