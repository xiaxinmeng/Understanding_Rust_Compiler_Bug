plain
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error[E0382]: borrow of moved value: `split`
     |
     |
2242 |     let mut split = arr.split(|v| v % 2 == 0);
     |         --------- move occurs because `split` has type `std::slice::Split<'_, i32, [closure@library/core/tests/slice.rs:2242:31: 2242:45]>`, which does not implement the `Copy` trait
...
2246 |     split.for_each(|_| {});
     |           ---------------- `split` moved due to this method call
2247 |     assert_eq!(split.as_slice(), &[]);
     |                ^^^^^^^^^^^^^^^^ value borrowed here after move
     |
note: this function takes ownership of the receiver `self`, which moves `split`
     |
     |
726  |     fn for_each<F>(self, f: F)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:01:30
