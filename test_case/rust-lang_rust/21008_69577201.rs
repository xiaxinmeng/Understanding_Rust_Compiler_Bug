 rust
no-method-suggested-traits.rs:23:10: 23:18 error: type `u32` does not implement any method in scope named `method`
no-method-suggested-traits.rs:23     1u32.method();
                                          ^~~~~~~~
no-method-suggested-traits.rs:23:10: 23:18 help: methods from traits can only be called if the trait is implemented and in scope; the following traits define a method `method`:
no-method-suggested-traits.rs:23     1u32.method();
                                          ^~~~~~~~
no-method-suggested-traits.rs:17:5: 19:6 help: candidate #1: `foo::Bar`
no-method-suggested-traits.rs:17     trait Bar { //~ HELP `foo::Bar`
no-method-suggested-traits.rs:18         fn method(&self);
no-method-suggested-traits.rs:19     }
no-method-suggested-traits.rs:23:10: 23:18 help: candidate #2: `no_method_suggested_traits::foo::PubPub`
no-method-suggested-traits.rs:23     1u32.method();
                                          ^~~~~~~~
no-method-suggested-traits.rs:23:10: 23:18 help: candidate #3: `no_method_suggested_traits::reexport::Reexported`
no-method-suggested-traits.rs:23     1u32.method();
                                          ^~~~~~~~
error: aborting due to previous error
