plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0432]: unresolved import `crate::clean::ToSource`
  --> src/librustdoc/clean/inline.rs:18:87
   |
18 |     self, utils, Attributes, AttributesExt, FakeDefId, GetDefId, NestedAttributesExt, ToSource,
   |                                                                                       ^^^^^^^^ no `ToSource` in `clean`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustdoc`
