
error[E0119]: conflicting implementations of trait `std::convert::TryFrom<()>` for type `S`
   --> main.rs:10:1
    |
10  |   impl TryFrom<()> for S {
    |   ^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `S`
    |
   ::: /home/lym/rust/rust/library/core/src/convert/mod.rs:598:1
    |
598 | / impl<T, U> const TryFrom<U> for T
599 | | where
600 | |     U: ~const Into<T>,
601 | | {
...   |
606 | |     }
607 | | }
    | |_- first implementation here
    |
    = note: conflicting implementation in crate `core`:
            - impl<T, U> TryFrom<U> for T
              where U: Into<T>;

