
no-method-suggested-traits.rs:22:10: 22:18 error: type `u32` does not implement any method in scope named `method`
no-method-suggested-traits.rs:22     1u32.method();
                                          ^~~~~~~~
no-method-suggested-traits.rs:22:10: 22:18 help: methods from traits can only be called if the trait is implemented and in scope; the following traits define a method `method`:
no-method-suggested-traits.rs:22     1u32.method();
                                          ^~~~~~~~
no-method-suggested-traits.rs:18:6: 18:6 help: candidate #1: `foo::Bar`
no-method-suggested-traits.rs:22:18: 22:18 help: candidate #2: `no_method_suggested_traits::foo::PubPub`
no-method-suggested-traits.rs:22:18: 22:18 help: candidate #3: `no_method_suggested_traits::bar::PubPriv`
no-method-suggested-traits.rs:22:18: 22:18 help: candidate #4: `no_method_suggested_traits::qux::PrivPub`
no-method-suggested-traits.rs:22:18: 22:18 help: candidate #5: `no_method_suggested_traits::quz::PrivPriv`
no-method-suggested-traits.rs:22:18: 22:18 help: candidate #6: `no_method_suggested_traits::reexport::Reexported`
no-method-suggested-traits.rs:22:18: 22:18 help: candidate #7: `no_method_suggested_traits::reexport::Reexported`
error: aborting due to previous error
