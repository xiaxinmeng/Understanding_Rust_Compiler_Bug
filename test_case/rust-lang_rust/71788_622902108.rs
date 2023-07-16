rust
trait Trait: ?Sized {
    fn func() -> Struct<Self>;    
}

struct Struct<T: Sized>{
    _t: std::marker::PhantomData<*const T>,
}
