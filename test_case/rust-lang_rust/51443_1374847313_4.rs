console
error[E0321]: cross-crate traits with a default impl, like `Sync`, can only be implemented for a struct/enum type, not `dyn Trait`
 --> src/main.rs
  |
  | unsafe impl Sync for dyn Trait + '_ {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't implement cross-crate trait with a default impl for non-struct/enum type
