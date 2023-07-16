
warning: the trait `anymap::any::CloneToAny` cannot be made into an object
  --> /home/.cargo/registry/src/github.com-1ecc6299db9ec823/anymap-0.12.1/src/any.rs:21:5
   |
21 |     fn clone_to_any_send_sync(&self) -> Box<CloneAny + Send + Sync> where Self: Send + Sync;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #51443 <https://github.com/rust-lang/rust/issues/51443>
   = note: method `clone_to_any_send_sync` references the `Self` type in where clauses
