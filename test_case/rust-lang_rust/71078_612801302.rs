
static  FOO: foo::Foo = {
    let mut _0: foo::Foo;                // return place in scope 0 at src/main.rs:8:17: 8:25
    let mut _1: &foo::Foo;               // in scope 0 at src/main.rs:8:28: 8:31

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at src/main.rs:8:28: 8:31
        _1 = const Scalar(alloc0+0): &foo::Foo; // bb0[1]: scope 0 at src/main.rs:8:28: 8:31
                                         // ty::Const
                                         // + ty: &foo::Foo
                                         // + val: Value(Scalar(alloc0+0))
                                         // mir::Constant
                                         // + span: src/main.rs:8:28: 8:31
                                         // + literal: Const { ty: &foo::Foo, val: Value(Scalar(alloc0+0)) }
        _0 = (*_1);                      // bb0[2]: scope 0 at src/main.rs:8:28: 8:31
        StorageDead(_1);                 // bb0[3]: scope 0 at src/main.rs:8:30: 8:31
        return;                          // bb0[4]: scope 0 at src/main.rs:8:1: 8:32
    }
}
