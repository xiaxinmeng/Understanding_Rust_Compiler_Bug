
error: cannot use constants which depend on generic parameters in types
   --> library/core/src/sync/atomic.rs:861:23
    |
861 |         let [] = [(); align_of::<Self>() - align_of::<*mut T>()];
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D const-evaluatable-unchecked` implied by `-D warnings`
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
