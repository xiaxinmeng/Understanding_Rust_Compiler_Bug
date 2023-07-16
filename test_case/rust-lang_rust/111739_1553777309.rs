rust
use std::ops::Deref;
use std::rc::Rc;

struct Value<T>(T);

pub trait Wrap<T> {
    fn wrap() -> Self;
}

impl<R, A1, A2> Wrap<fn(A1, A2) -> R> for Value<fn(A1, A2) -> R> {
    fn wrap() -> Self {
        todo!()
    }
}

impl<F, R, A1, A2> Wrap<F> for Value<Rc<dyn Fn(A1, A2) -> R>> {
    fn wrap() -> Self {
        todo!()
    }
}

impl<F> Deref for Value<Rc<F>> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

fn main() {
    let var_fn = Value::wrap();
    let _ = var_fn.clone();
}
