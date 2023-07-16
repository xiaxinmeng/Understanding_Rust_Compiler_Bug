rust
let x = a.skip_binder();
let y = do_stuff(x);
x.with_bound_value(y)
