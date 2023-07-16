plain
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
error: `cfg` predicate key must be an identifier
     |
339  | macro_rules! dbg {
     | ---------------- in this expansion of `dbg!`
...
...
353  |                 #[cfg($crate::debug_assertions)]
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: library/alloc/tests/vec.rs:1916:18
     |
1916 |     let vtable = dbg!(ptr_metadata(raw_dyn));


error: `cfg` predicate key must be an identifier
     |
339  | macro_rules! dbg {
     | ---------------- in this expansion of `dbg!`
...
...
353  |                 #[cfg($crate::debug_assertions)]
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: library/alloc/tests/vec.rs:1921:5
     |
1921 |     dbg!(ptr_metadata(vec[0]));


error: `cfg` predicate key must be an identifier
    |
339 | / macro_rules! dbg {
340 | |     // NOTE: We cannot use `concat!` to make a static string as a format argument
340 | |     // NOTE: We cannot use `concat!` to make a static string as a format argument
341 | |     // of `eprintln!` because `file!` could contain a `{` or
342 | |     // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
353 | |                 #[cfg($crate::debug_assertions)]
    | |                       ^^^^^^^^^^^^^^^^^^^^^^^^
...   |
362 | |     };
362 | |     };
363 | | }
    | |_- in this expansion of `dbg!`
    |
   ::: library/std/src/sys/unix/process/process_unix/tests.rs:49:9
    |
49  |           dbg!(st);


error: `cfg` predicate key must be an identifier
    |
339 | / macro_rules! dbg {
340 | |     // NOTE: We cannot use `concat!` to make a static string as a format argument
340 | |     // NOTE: We cannot use `concat!` to make a static string as a format argument
341 | |     // of `eprintln!` because `file!` could contain a `{` or
342 | |     // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
353 | |                 #[cfg($crate::debug_assertions)]
    | |                       ^^^^^^^^^^^^^^^^^^^^^^^^
...   |
362 | |     };
362 | |     };
363 | | }
    | |_- in this expansion of `dbg!`
    |
   ::: library/std/src/sys/unix/process/process_unix/tests.rs:52:5
    |
52  |       dbg!(&got);


error: `cfg` predicate key must be an identifier
    |
339 | / macro_rules! dbg {
340 | |     // NOTE: We cannot use `concat!` to make a static string as a format argument
340 | |     // NOTE: We cannot use `concat!` to make a static string as a format argument
341 | |     // of `eprintln!` because `file!` could contain a `{` or
342 | |     // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
353 | |                 #[cfg($crate::debug_assertions)]
    | |                       ^^^^^^^^^^^^^^^^^^^^^^^^
...   |
362 | |     };
362 | |     };
363 | | }
    | |_- in this expansion of `dbg!`
    |
   ::: library/std/src/sys/unix/process/process_unix/tests.rs:54:5
    |
54  |       dbg!(status);

error: could not compile `alloc` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `std` due to 3 previous errors
