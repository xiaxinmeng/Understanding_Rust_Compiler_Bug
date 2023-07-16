rust
fn main() {}

pub trait Strategy {
    type Value;
}

fn prop_map<O, F: Fn(S::Value) -> O, S: Strategy>(_: S, _: F) -> Map<S, F> {
    loop {}
}

fn boxed<S: 'static + Strategy>(_: S) -> *const dyn Strategy<Value = S::Value> {
    loop {}
}

pub struct Map<S, F> {
    x: std::marker::PhantomData<(S, F)>,
}

impl<S: Strategy, O, F: Fn(S::Value) -> O> Strategy for Map<S, F> {
    type Value = O;
}

fn cloned<'a, T: 'a, I: Iterator<Item = &'a T>>(x: I) -> I {
    x
}

fn lift1_with<'a, T, A, S>(base: S) -> *const dyn Strategy<Value = A>
where
    T: 'static,
    A: 'static + Iterator<Item = &'a T>,
    S: Strategy<Value = A> + 'static,
{
    boxed(prop_map(base, cloned))
}
