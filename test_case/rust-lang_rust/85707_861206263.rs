
warning: trait method `try_into` will become ambiguous in Rust 2021
  --> ffishim/src/field.rs:40:13
   |
40 |             crate::types::switch(&self.ty).try_into(&self.ty, receiver)
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: disambiguate the associated function: `types::Behavior::try_into(&**crate::types::switch(&self.ty), &self.ty, receiver)`
   |
   = note: `-W future-prelude-collision` implied by `-W rust-2021-compatibility`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
   = note: for more information, see issue #85684 <https://github.com/rust-lang/rust/issues/85684>
