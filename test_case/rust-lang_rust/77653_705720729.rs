rust
trait Proj {
    type S;
}

trait Base<T> {
    fn is_base(&self);
}
trait Der<B: Proj>: Base<B::S> {
    fn is_der(&self);
}

fn f<P: Proj>(obj: &dyn Der<P>) {
    // Uncomment for ICE on stable
    // obj.is_base();
    obj.is_der();
}

impl Proj for () {
    type S = ();
}

pub fn main() {
    let x: fn(_) = f::<()>;
}
