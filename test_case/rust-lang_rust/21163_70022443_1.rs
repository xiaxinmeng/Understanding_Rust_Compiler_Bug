 rust
#[inline]
fn ge(&self, __arg_0: &Foo) -> ::bool {
    match (&*self, &*__arg_0) {
        (&Foo::Bar(ref __self_0), &Foo::Bar(ref __arg_1_0)) =>
        (*__self_0) > (*__arg_1_0) ||
            !((*__arg_1_0) > (*__self_0)) && true,
        (&Foo::Baz, &Foo::Baz) => true,
        _ => {
            let __self_vi =  // has type `usize`
                match *self { Foo::Bar(..) => 0us, Foo::Baz(..) => 1us, };
            let __arg_1_vi =  // has type `usize`
                match *__arg_0 {
                    Foo::Bar(..) => 0us,
                    Foo::Baz(..) => 1us,
                };
            __self_vi.ge(&__arg_1_vi)  // <-- method call
        }
    }
}
