plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0609]: no field `def_id` on type `types::Item`
  --> src/librustdoc/formats/mod.rs:51:30
   |
51 |         match self.impl_item.def_id {
   |
   |
   = note: available fields are: `name`, `attrs`, `visibility`, `kind`, `item_id`, `cfg`
error[E0609]: no field `def_id` on type `types::Item`
  --> src/librustdoc/formats/mod.rs:56:94
   |
   |
56 |                 panic!("Unexpected ItemId::Primitive in expect_def_id: {:?}", self.impl_item.def_id)
   |
   |
   = note: available fields are: `name`, `attrs`, `visibility`, `kind`, `item_id`, `cfg`
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:02:40
