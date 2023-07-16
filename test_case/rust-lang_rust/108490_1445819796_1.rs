
fn a<F: for<'s> Fn(PhantomData<&'s ()>) -> Vec<&'s i32>>(f: F)
