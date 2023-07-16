plain
    Checking rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
error[E0631]: type mismatch in function arguments
    --> compiler/rustc_metadata/src/rmeta/decoder.rs:1641:80
     |
1641 |         let update = Some(new_extern_crate.rank()) > extern_crate.as_ref().map(ExternCrate::rank);
     |                                                                            |   |
     |                                                                            |   expected due to this
     |                                                                            |   found signature defined here
     |                                                                            required by a bound introduced by this call
     |                                                                            required by a bound introduced by this call
     |
     = note: expected function signature `fn(rustc_session::cstore::ExternCrate) -> _`
                found function signature `for<'a> fn(&'a rustc_session::cstore::ExternCrate) -> _`
note: required by a bound in `std::option::Option::<T>::map`
     |
     |
1095 |         F: ~const FnOnce(T) -> U,
     |            ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
For more information about this error, try `rustc --explain E0631`.
error: could not compile `rustc_metadata` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_metadata` due to previous error
error: could not compile `rustc_metadata` due to previous error
error[E0507]: cannot move out of a shared reference
   --> compiler/rustc_infer/src/infer/resolve.rs:234:32
    |
234 |               ty::ReVar(_) => Ok(self
235 | |                 .infcx
236 | |                 .lexical_region_resolutions
237 | |                 .borrow()
238 | |                 .as_ref()
238 | |                 .as_ref()
239 | |                 .expect("region resolution not performed")
    | |                  |                                       |
    | |                  |                                       |
    | |__________________|_______________________________________help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents
    |                    |                                       move occurs because value has type `std::option::Option<LexicalRegionResolutions<'_>>`, which does not implement the `Copy` trait
    |                    value moved due to this method call
    |
note: `std::option::Option::<T>::expect` takes ownership of the receiver `self`, which moves value
    |
910 |     pub const fn expect(self, msg: &str) -> T {
    |                         ^^^^
    |                         ^^^^
help: you can `clone` the value and consume it, but this might not be your desired behavior
    |
239 |                 .clone().expect("region resolution not performed")

For more information about this error, try `rustc --explain E0507`.
error: could not compile `rustc_infer` due to previous error
error: could not compile `rustc_infer` due to previous error
