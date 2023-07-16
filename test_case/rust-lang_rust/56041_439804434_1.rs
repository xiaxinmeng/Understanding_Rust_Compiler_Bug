rust
    let ref_pair = Box::leak(box_pair) as *mut PairFoo;
    let ptr_foo = unsafe { &mut (*ref_pair).fst as *mut Foo };
