rust
let x = a.skip_binder();
let y = do_stuff(x);
ty::Binder::bind(y)
