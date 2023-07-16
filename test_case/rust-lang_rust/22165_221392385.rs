
#![crate_type = "lib"]

trait Tuple: Sized {
    type Array: Toa<Item=Self>;
}

trait Toa {
    type Item;

    fn push(&mut self, Self::Item);
    fn with_capacity(usize) -> Self;
}

#[cfg(not(works))]
fn f<T: Tuple>(t: T) -> T::Array {
    let mut toa = Toa::with_capacity(1);
    toa.push(t);  //~ error: the type of this value must be known in this context
    toa
}

#[cfg(works)]
fn g<T: Tuple>(t: T) -> T::Array {
    let mut toa = Toa::with_capacity(1);
    Toa::push(&mut toa, t);
    toa
}

