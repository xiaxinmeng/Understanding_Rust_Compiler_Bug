plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0277]: a value of type `HashSet<std::string::String, BuildHasherDefault<FxHasher>>` cannot be built from an iterator over elements of type `Symbol`
     |
     |
540  |             .collect::<FxHashSet<String>>();
     |              ^^^^^^^ value of type `HashSet<std::string::String, BuildHasherDefault<FxHasher>>` cannot be built from `std::iter::Iterator<Item=Symbol>`
     |
     = help: the trait `FromIterator<Symbol>` is not implemented for `HashSet<std::string::String, BuildHasherDefault<FxHasher>>`
note: required by a bound in `std::iter::Iterator::collect`
     |
     |
1737 |     fn collect<B: FromIterator<Self::Item>>(self) -> B
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `std::iter::Iterator::collect`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
