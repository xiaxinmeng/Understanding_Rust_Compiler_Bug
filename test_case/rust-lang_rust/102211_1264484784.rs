rs
struct Type<'a, 'b>(&'a &'b (), ::core::marker::PhantomData<*mut ()>);

// Note: if this had an *implicit* `'b : 'a` things would work.
unsafe impl<'b : 'a, 'a> Send for Type<'a, 'b> {}

fn foo() -> impl Send { async {
    let local = ();
    let r = &&local;
    async {}.await;
    let it = Type(r, <_>::default());
    async {}.await;
}}
