
foo.rs:3:5: 3:6 error: the trait `core::marker::Sized` is not implemented for the type `[isize]` [E0277]
foo.rs:3 let y = *x;        // error: mismatched types: expected `<generic #1>`, found `[int]`
             ^
foo.rs:3:5: 3:6 note: `[isize]` does not have a constant size known at compile-time
foo.rs:3 let y = *x;        // error: mismatched types: expected `<generic #1>`, found `[int]`
             ^
foo.rs:4:5: 4:6 error: the trait `core::marker::Sized` is not implemented for the type `[isize]` [E0277]
foo.rs:4 let y: [isize] = *x; // error: mismatched types: expected `[int]`, found `<generic #3>`
             ^
foo.rs:4:5: 4:6 note: `[isize]` does not have a constant size known at compile-time
foo.rs:4 let y: [isize] = *x; // error: mismatched types: expected `[int]`, found `<generic #3>`
             ^
foo.rs:7:5: 7:6 error: the trait `core::marker::Sized` is not implemented for the type `str` [E0277]
foo.rs:7 let s = *"hello world";
             ^
foo.rs:7:5: 7:6 note: `str` does not have a constant size known at compile-time
foo.rs:7 let s = *"hello world";
             ^
foo.rs:8:5: 8:6 error: the trait `core::marker::Sized` is not implemented for the type `str` [E0277]
foo.rs:8 let s: str = *"hello world";
             ^
foo.rs:8:5: 8:6 note: `str` does not have a constant size known at compile-time
foo.rs:8 let s: str = *"hello world";
             ^
error: aborting due to 4 previous errors
