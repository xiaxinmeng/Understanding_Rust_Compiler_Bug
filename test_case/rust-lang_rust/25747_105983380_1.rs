 rust
    let x: RefCell<Result<u32, ()>> = RefCell::new(Ok(5));
    let b: Option<Ref<u32>> = Ref::generalized_map(x.borrow(), |r, new_ref| match *r {
        Ok(ref value) => Some(new_ref(value)),
        Err(_) => None,
    });
    assert_eq!(*b.unwrap(), 5);
}
