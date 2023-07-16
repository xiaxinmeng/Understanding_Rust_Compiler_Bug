plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `clean` found for tuple `(rustc_middle::ty::TraitRef<'_>, &[_; 0])` in the current scope
   --> src/librustdoc/clean/mod.rs:142:58
    |
142 |                     PolyTrait { trait_: (trait_ref, &[]).clean(cx), generic_params: vec![] },
    |                                                          ^^^^^ method not found in `(rustc_middle::ty::TraitRef<'_>, &[_; 0])`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `clean::Clean` defines an item `clean`, perhaps you need to implement it
   --> src/librustdoc/clean/mod.rs:54:1
    |
54  | crate trait Clean<T> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
