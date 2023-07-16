plain
    Checking rand v0.8.5
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
error[E0596]: cannot borrow `rng` as mutable, as it is not declared as mutable
    |
    |
395 |     let rng = &mut rng;
    |         --- help: consider changing this to be mutable: `mut rng`
...
451 |     v.sort_by(|_, _| *[Less, Equal, Greater].choose(&mut rng).unwrap());
    |                                                     ^^^^^^^^ cannot borrow as mutable
For more information about this error, try `rustc --explain E0596`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:57
