
error[E0119]: conflicting implementations of trait `convert::AsRef<[&_]>` for type `&_`:
    --> src/libcore/slice/mod.rs:5532:1
     |
5532 |   impl<T: ?Sized> AsRef<[T]> for T {
     |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`
     | 
    ::: src/libcore/convert/mod.rs:504:1
     |
504  | / impl<T: ?Sized, U: ?Sized> AsRef<U> for &T
505  | | where
506  | |     T: AsRef<U>,
507  | | {
...    |
510  | |     }
511  | | }
     | |_- first implementation here
