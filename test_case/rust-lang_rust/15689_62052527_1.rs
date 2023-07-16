 rust
fn partial_cmp(&self, __arg_0: &Foo) ->
     ::std::option::Option<::std::cmp::Ordering> {
        match *__arg_0 {
            Foo(ref __self_1_0) =>
            match *self {
                Foo(ref __self_0_0) => {
                    let __test = (*__self_0_0).partial_cmp(&(*__self_1_0));
                    if __test == ::std::option::Some(::std::cmp::Equal) {
                        ::std::option::Some(::std::cmp::Equal)
                    } else { __test }
                }
            },
        }
    }
