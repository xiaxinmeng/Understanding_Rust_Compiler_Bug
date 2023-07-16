plain
    Checking rustc-demangle v0.1.21
error: overly complex generic constant
   --> library/alloc/src/raw_vec.rs:292:31
    |
292 |             let _: () = const { assert!(mem::size_of::<T>() % mem::align_of::<T>() == 0) };
    |                               ^^--------------------------------------------------------^^
    |                                 control flow is not supported in generic constants
    |
    |
    = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
   --> library/alloc/src/raw_vec.rs:486:27
    |
    |
486 |         let _: () = const { assert!(mem::size_of::<T>() % mem::align_of::<T>() == 0) };
    |                           ^^--------------------------------------------------------^^
    |                             control flow is not supported in generic constants
    |
    |
    = help: consider moving this anonymous constant into a `const` function
error: could not compile `alloc` due to 2 previous errors
Build completed unsuccessfully in 0:00:18
