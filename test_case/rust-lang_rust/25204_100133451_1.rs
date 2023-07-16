
<anon>:25:1: 25:60 error: the trait `core::marker::Sized` is not implemented for the type `T` [E0277]
<anon>:25 fn weird<T: ?Sized>(_: Option<T>) where Option<T>: Sized {}
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:25:1: 25:60 note: `T` does not have a constant size known at compile-time
<anon>:25 fn weird<T: ?Sized>(_: Option<T>) where Option<T>: Sized {}
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
