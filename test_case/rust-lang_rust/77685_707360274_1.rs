rust
let x = a.skip_binder();
let y = do_stuff(x);
y.with_binders(&x)
