plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0282]: type annotations needed
    --> library/core/src/slice/mod.rs:1673:11
     |
1673 |         (&*(a.as_ptr() as _), b)
     |
     |
     = note: type must be known at this point
error[E0282]: type annotations needed
    --> library/core/src/slice/mod.rs:1701:14
     |
     |
1701 |         (&mut*(a.as_mut_ptr() as _), b)
     |
     |
     = note: type must be known at this point
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0282`.
error: could not compile `core`
