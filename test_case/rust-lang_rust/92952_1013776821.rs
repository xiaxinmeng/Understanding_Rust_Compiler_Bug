plain
[RUSTC-TIMING] rustc_codegen_ssa test:false 46.570
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
[RUSTC-TIMING] rustc_plugin_impl test:false 5.140
   Compiling rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0277]: a value of type `HashSet<std::string::String, BuildHasherDefault<FxHasher>>` cannot be built from an iterator over elements of type `Symbol`
     |
     |
540  |             .collect::<FxHashSet<String>>();
     |              ^^^^^^^ value of type `HashSet<std::string::String, BuildHasherDefault<FxHasher>>` cannot be built from `std::iter::Iterator<Item=Symbol>`
     |
     = help: the trait `FromIterator<Symbol>` is not implemented for `HashSet<std::string::String, BuildHasherDefault<FxHasher>>`
note: required by a bound in `collect`
For more information about this error, try `rustc --explain E0277`.
[RUSTC-TIMING] rustc_typeck test:false 6.038
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
