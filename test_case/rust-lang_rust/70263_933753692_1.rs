rust
use std::marker::PhantomData;

// A trait that is essentially a synonym of a Fn trait
pub trait PseudoFn {
    type Arg;

    fn apply(&self, arg: Self::Arg) -> Self::Arg;
}

pub struct FnWrapper<Arg, F>(F, PhantomData<Arg>);

// Automatically implement PseudoFn for Fn
impl<Arg, F> From<F> for FnWrapper<Arg, F>
where
    F: Fn(Arg) -> Arg + Sized,
{
    fn from(f: F) -> Self {
        FnWrapper(f, Default::default())
    }
}

impl<Arg, F> PseudoFn for FnWrapper<Arg, F>
where
    F: Fn(Arg) -> Arg,
{
    type Arg = Arg;

    fn apply(&self, arg: Self::Arg) -> Self::Arg {
        (self.0)(arg)
    }
}

// A function that can be called using the Fn trait
pub fn ok<Arg, F>(f: F, arg: Arg) -> Arg
where
    F: Fn(Arg) -> Arg,
{
    FnWrapper::from(f).apply(arg)
}

// A function like `ok` that uses the PseudoFn trait
// but can't be called the same way
pub fn bad<Arg, F, W: Into<FnWrapper<Arg, F>>>(w: W, arg: Arg) -> Arg
where
    F: Fn(Arg) -> Arg,
{
    w.into().apply(arg)
}

// Wrapper around `bad` that compiles and can be called
pub fn call_bad<Arg, F>(f: F, arg: Arg) -> Arg
where
    F: Fn(Arg) -> Arg,
{
    bad(f, arg)
}

fn do_test() {
    ok(|i: &i32| i, &0); // This compiles.
    call_bad(|i: &i32| i, &0); // So does this.
    bad(|i: &i32| i, &0); // This does't.
}
