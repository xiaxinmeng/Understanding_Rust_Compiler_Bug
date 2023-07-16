rust
    let ref_pair: &mut PairFoo = Box::leak(box_pair);
    let ref_foo: &mut Foo = &mut ref_pair.fst;
    let ptr_foo: *mut Foo = ref_foo as *mut Foo;
