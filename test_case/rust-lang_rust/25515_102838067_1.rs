 rust
    let ptr = std::mem::transmute::<_, *mut Holder<Foo>>(Box::new(Holder(Foo(0))));
    let big_ptr: *mut Holder<Send> = ptr;
    std::intrinsics::drop_in_place(&mut (*big_ptr).0 as *mut _);
