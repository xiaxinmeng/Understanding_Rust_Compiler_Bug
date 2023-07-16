

error[E0275]: overflow evaluating the requirement `rule::Expr: struct_delta_trait::DeltaOps`
  --> src/parser.rs:26:10
   |
26 | #[derive(struct_delta_derive::Delta)]
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: required because of the requirements on the impl of `struct_delta_trait::DeltaOps` for `std::vec::Vec<rule::Expr>`
   = note: required because of the requirements on the impl of `struct_delta_trait::DeltaOps` for `rule::Rule`
   = note: required because of the requirements on the impl of `struct_delta_trait::DeltaOps` for `std::borrow::Cow<'static, rule::Rule>`
   = note: required because of the requirements on the impl of `struct_delta_trait::DeltaOps` for `stack::StackFrame`
   = note: required because of the requirements on the impl of `struct_delta_trait::DeltaOps` for `std::vec::Vec<stack::StackFrame>`
   = note: required because of the requirements on the impl of `struct_delta_trait::DeltaOps` for `stack::Stack`
   = help: see issue #48214
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
