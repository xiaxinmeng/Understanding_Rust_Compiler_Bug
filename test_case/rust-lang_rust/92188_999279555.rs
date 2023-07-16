plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0277]: the size for values of type `Self` cannot be known at compilation time
   --> src/librustdoc/clean/types.rs:883:17
    |
883 |     fn has_word(self, word: Symbol) -> bool {
    |
    = help: unsized fn params are gated as an unstable feature
help: consider further restricting `Self`
    |
    |
883 |     fn has_word(self, word: Symbol) -> bool where Self: std::marker::Sized {
    |                                             ++++++++++++++++++++++++++++++
help: function arguments must have a statically known size, borrowed types always have a known size
    |
883 |     fn has_word(&self, word: Symbol) -> bool {

error[E0277]: the size for values of type `Self` cannot be known at compilation time
   --> src/librustdoc/clean/types.rs:884:54
    |
    |
884 |         <Self as NestedAttributesExt>::get_word_attr(self, word).is_some()
    |
    = note: all function arguments must have a statically known size
    = help: unsized fn params are gated as an unstable feature
help: consider further restricting `Self`
help: consider further restricting `Self`
    |
883 |     fn has_word(self, word: Symbol) -> bool where Self: std::marker::Sized {

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:03:14
