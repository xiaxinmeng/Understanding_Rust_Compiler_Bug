plain
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error[E0282]: type annotations needed for `realstd::vec::Vec<T>`
   |
   |
47 |     let mut buf = Vec::with_capacity(1024);
   |         -------   ^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `T`
   |         |
   |         consider giving `buf` the explicit type `realstd::vec::Vec<T>`, where the type parameter `T` is specified
error[E0283]: type annotations needed
   --> library/std/src/io/impls/tests.rs:53:24
    |
    |
53  |             let _ = wr.write_all(&src);
    |                     ---^^^^^^^^^------
    |                     |  |
    |                     |  cannot infer type for mutable reference `&mut [_]`
    |                     this method call resolves to `core::result::Result<(), io::error::Error>`
    |
note: multiple `impl`s satisfying `&mut [_]: io::Write` found
    |
    |
334 | impl Write for &mut [u8] {
...
...
424 | impl Write for &mut [mem::MaybeUninit<u8>] {

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
error: could not compile `std` due to 2 previous errors
