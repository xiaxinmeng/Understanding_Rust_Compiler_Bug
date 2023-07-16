
#[doc(hidden)]
macro_rules! my_wrapper{
    ($n:ident,$f:ident) =>{

/// Wrapper that calls `$f` before doing ...
struct $n<T> where T: ...;

impl<T> $n<T> where T: ... {
    ... make something call $f ...
}

    }
}
