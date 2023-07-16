rust
let x = AtomicI32::new(0);

let y = &x;

let p = x.as_mut_ptr();
// SAFETY: The following line is safe because the current thread is the only one with access to `x`, so no data race occurs
unsafe{p.write(1);}
``