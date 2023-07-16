 rust
impl<A,B> SomeTrait<Option<B>> for Foo<A> {
    type Output = Result<A, IoError>;
}

// Foo<A>::Output is different for each A

struct Bar<T> {
    _m: PhantomData<T>
}

// Bar<T>::Maker is different for each T
