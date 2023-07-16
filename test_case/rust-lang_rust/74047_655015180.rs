
error[E0046]: not all trait items implemented, missing: `Error`, `try_from`
  --> src/lib.rs:15:1
   |
15 | impl TryFrom<OtherStream> for MyStream {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `Error`, `try_from` in implementation
   |
   = help: implement the missing item: `type Error = Type;`
   = help: implement the missing item: `fn try_from(_: T) -> std::result::Result<Self, <Self as std::convert::TryFrom<T>>::Error> { todo!() }`
