rust
use std::ops::Deref;
use std::rc::Rc;

struct Value<T>(T);

pub trait Wrap<T: ?Sized> {
    fn wrap(f: T) -> Self;
}

impl<R, A1, A2> Wrap<fn(A1, A2) -> R> for Value<fn(A1, A2) -> R> {}

impl<F, R, A1, A2> Wrap<F> for Value<Rc<dyn Fn(A1, A2) -> R>> {}

impl<F> Deref for Value<Rc<F>> {
    type Target = F;
}

fn fn_select(var_x: bool) -> i64 {
    let var_fn = Value::wrap();
    match var_fn.clone() {}
}
