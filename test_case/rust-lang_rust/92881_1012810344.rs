rust
#![feature(const_trait_impl, const_fn_trait_bound, const_mut_refs)]

trait X {
    fn x();
}

struct Y<T: X>(T);

impl<T: X> const Drop for Y<T> {
    fn drop(&mut self)
    where
        T: ~const X
    {
        T::x();
    }
}
