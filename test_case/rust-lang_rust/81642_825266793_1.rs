
error[E0119]: conflicting implementations of trait `convert::TryFrom<u8>`:
   --> library/core/src/convert/mod.rs:591:1
    |
591 | / impl<T, U> TryFrom<U> for T
592 | | where
593 | |     U: Into<T>,
594 | | {
...   |
599 | |     }
600 | | }
    | |_^ conflicting implementation
    | 
   ::: library/core/src/num/enums.rs:22:1
    |
22  |   impl <C> TryFrom<u8> for C where C: ConvertableEnum<Repr = u8> + Sized {
    |   ---------------------------------------------------------------------- first implementation here
