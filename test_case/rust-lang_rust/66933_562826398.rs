
		 error[E0277]: arrays only have std trait implementations for lengths 0..=32
   --> $DIR/core-traits-no-impls-length-33.rs:8:19
    |
 LL |     let mut set = HashSet::new();
    |                   ^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
    |
    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
    = note: required by `std::collections::HashSet::<T>::new`
