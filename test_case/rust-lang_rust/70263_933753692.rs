rust
// A trait that is essentially a synonym of a Fn trait
pub trait PseudoFn<T> {}

// Automatically implement PseudoFn for Fn
impl<T, F> PseudoFn<T> for F where F: Fn(T) -> T {}

// A function that can be called using the Fn trait
pub fn ok<Arg, F>(_f: F)
where
    F: Fn(Arg) -> Arg,
{
}

// A function like `ok` that uses the PseudoFn trait
// but can't be called the same way
pub fn bad<Arg, F>(_f: F)
where
    F: PseudoFn<Arg>,
{
}

// Wrapper around `bad` that compiles and can be called
pub fn call_bad<Arg, F>(f: F)
where
    F: Fn(Arg) -> Arg,
{
    bad(f);
}

fn do_test() {
    ok(|i: &i32| i);       // This compiles.
    call_bad(|i: &i32| i); // So does this.
    bad(|i: &i32| i);      // This does't.
}
