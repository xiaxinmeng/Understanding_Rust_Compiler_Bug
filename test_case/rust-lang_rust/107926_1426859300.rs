plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
    --> src/librustdoc/clean/types.rs:1338:9
     |
1337 |     pub(crate) fn type_builder(&self) -> bool {
     |                                          ---- expected `bool` because of return type
1338 |         self.inputs.values.get(0).and_then(|v| v.to_self())
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found enum `Option`
     = note: expected type `bool`
     = note: expected type `bool`
                found enum `std::option::Option<clean::types::SelfTy>`
help: use `Option::is_some` to test if the `Option` has a value
     |
1338 |         self.inputs.values.get(0).and_then(|v| v.to_self()).is_some()

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:01:51
