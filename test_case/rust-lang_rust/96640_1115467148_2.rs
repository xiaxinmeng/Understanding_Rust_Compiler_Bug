rust
struct __SerializeWith<'__a, 'a: '__a> {
    values: (&'__a &'a String,),
    phantom: _serde::__private::PhantomData<Foo<'a>>,
}
