plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0507]: cannot move out of a shared reference
   --> compiler/rustc_infer/src/infer/resolve.rs:209:32
    |
209 |               ty::ReVar(_) => Ok(self
210 | |                 .infcx
211 | |                 .lexical_region_resolutions
212 | |                 .borrow()
213 | |                 .as_ref()
213 | |                 .as_ref()
214 | |                 .expect("region resolution not performed")
    | |__________________|_______________________________________|
    | |__________________|_______________________________________|
    |                    |                                       move occurs because value has type `Option<LexicalRegionResolutions<'_>>`, which does not implement the `Copy` trait
    |                    value moved due to this method call
    |
note: this function takes ownership of the receiver `self`, which moves value
    |
735 |     pub const fn expect(self, msg: &str) -> T {
    |                         ^^^^
    |                         ^^^^
help: consider calling `.as_ref()` to borrow the type's contents
    |
214 |                 .as_ref().expect("region resolution not performed")

For more information about this error, try `rustc --explain E0507`.
error: could not compile `rustc_infer` due to previous error
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_infer` due to previous error
error[E0507]: cannot move out of a shared reference
    --> compiler/rustc_resolve/src/lib.rs:1577:44
     |
1577 |         for (trait_name, trait_binding) in traits.as_ref().unwrap().iter() {
     |                                            ^^^^^^^^^^^^^^^^--------
     |                                            |               |
     |                                            |               value moved due to this method call
     |                                            move occurs because value has type `Option<std::boxed::Box<[(rustc_span::symbol::Ident, &NameBinding<'_>)]>>`, which does not implement the `Copy` trait
     |
note: this function takes ownership of the receiver `self`, which moves value
     |
772  |     pub const fn unwrap(self) -> T {
     |                         ^^^^
     |                         ^^^^
help: consider calling `.as_ref()` to borrow the type's contents
     |
1577 |         for (trait_name, trait_binding) in traits.as_ref().as_ref().unwrap().iter() {

error: could not compile `rustc_resolve` due to previous error
error: could not compile `rustc_resolve` due to previous error
Build completed unsuccessfully in 0:02:12
