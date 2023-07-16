rust
trait Trait<'s, 't, 'u> {}

#[derive(Clone)]
struct ShimMethod3<T: CallWithShim2 + 'static>(
    pub &'static dyn for<'s> Fn(&'s mut T::Shim<dyn for<'t> Fn(&'s mut T::Shim<dyn for<'u> Trait<'s, 't, 'u>>)>),
);
