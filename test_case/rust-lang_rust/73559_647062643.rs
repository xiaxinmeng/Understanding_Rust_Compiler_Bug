rust
trait TypeFunc {
    type Output;
}

struct TF;
struct Out;
impl TypeFunc for TF {
    type Output = Out;
}

trait Trait {
    type Assoc: From<<TF as TypeFunc>::Output>;
}

fn foo<T: Trait>(_: T){}
