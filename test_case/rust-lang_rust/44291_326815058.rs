rust
#![feature(fn_traits, unboxed_closures)]

fn closure_to_fn_and_data<A, F: Fn<A> + 'static>(
    f: F,
) -> (
    unsafe extern "C" fn(*const Box<Fn<A, Output = F::Output>>, A) -> F::Output,
    *const Box<Fn<A, Output = F::Output>>,
) {
    unsafe extern "C" fn callback<A, F: Fn<A> + 'static>(
        closure: *const Box<Fn<A, Output = F::Output>>,
        args: A,
    ) -> F::Output {
        std::ops::Fn::call(&**closure, args)
    }

    (
        callback::<A, F>,
        Box::into_raw(Box::new(Box::new(f) as Box<Fn<A, Output = F::Output>>)) as _,
    )
}
