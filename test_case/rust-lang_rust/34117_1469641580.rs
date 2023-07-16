rust
make_foo(&mut &mut *result); // mismatched types: expected `*mut *mut Foo`, found `&mut &mut _`
