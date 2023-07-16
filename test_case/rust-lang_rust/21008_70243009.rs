
no-method-suggested-traits.rs:28:10: 28:18 error: type `u32` does not implement any method in scope named `method`
no-method-suggested-traits.rs:28     1u32.method();
                                          ^~~~~~~~
no-method-suggested-traits.rs:28:18: 28:18 help: methods from traits can only be called if the trait is in scope; the following traits are implemented and define a method `method`:
no-method-suggested-traits.rs:28:18: 28:18 help: candidate #1: `foo::Bar`
no-method-suggested-traits.rs:28:18: 28:18 help: candidate #2: `no_method_suggested_traits::foo::PubPub`
no-method-suggested-traits.rs:34:9: 34:17 error: type `char` does not implement any method in scope named `method`
no-method-suggested-traits.rs:34     'a'.method();
                                         ^~~~~~~~
no-method-suggested-traits.rs:34:17: 34:17 help: methods from traits can only be called if the trait is in scope; the following trait is implemented and defines a method `method`:
no-method-suggested-traits.rs:34:17: 34:17 help: candidate #1: `foo::Bar`
no-method-suggested-traits.rs:39:10: 39:18 error: type `i32` does not implement any method in scope named `method`
no-method-suggested-traits.rs:39     1i32.method();
                                          ^~~~~~~~
no-method-suggested-traits.rs:39:18: 39:18 help: methods from traits can only be called if the trait is in scope; the following trait is implemented and defines a method `method`:
no-method-suggested-traits.rs:39:18: 39:18 help: candidate #1: `no_method_suggested_traits::foo::PubPub`
no-method-suggested-traits.rs:44:10: 44:18 error: type `u64` does not implement any method in scope named `method`
no-method-suggested-traits.rs:44     1u64.method();
                                          ^~~~~~~~
no-method-suggested-traits.rs:44:18: 44:18 help: methods from traits can only be called if the trait is implemented and in scope; no such traits are but the following traits define a method `method`:
no-method-suggested-traits.rs:44:18: 44:18 help: candidate #1: `foo::Bar`
no-method-suggested-traits.rs:44:18: 44:18 help: candidate #2: `no_method_suggested_traits::foo::PubPub`
no-method-suggested-traits.rs:44:18: 44:18 help: candidate #3: `no_method_suggested_traits::bar::PubPriv`
no-method-suggested-traits.rs:44:18: 44:18 help: candidate #4: `no_method_suggested_traits::qux::PrivPub`
no-method-suggested-traits.rs:44:18: 44:18 help: candidate #5: `no_method_suggested_traits::quz::PrivPriv`
no-method-suggested-traits.rs:44:18: 44:18 help: candidate #6: `no_method_suggested_traits::reexport::Reexported`
no-method-suggested-traits.rs:44:18: 44:18 help: candidate #7: `no_method_suggested_traits::reexport::Reexported`
no-method-suggested-traits.rs:54:10: 54:19 error: type `u64` does not implement any method in scope named `method2`
no-method-suggested-traits.rs:54     1u64.method2();
                                          ^~~~~~~~~
no-method-suggested-traits.rs:54:19: 54:19 help: methods from traits can only be called if the trait is implemented and in scope; no such traits are but the following trait defines a method `method2`:
no-method-suggested-traits.rs:54:19: 54:19 help: candidate #1: `foo::Bar`
no-method-suggested-traits.rs:58:10: 58:19 error: type `u64` does not implement any method in scope named `method3`
no-method-suggested-traits.rs:58     1u64.method3();
                                          ^~~~~~~~~
no-method-suggested-traits.rs:58:19: 58:19 help: methods from traits can only be called if the trait is implemented and in scope; no such traits are but the following trait defines a method `method3`:
no-method-suggested-traits.rs:58:19: 58:19 help: candidate #1: `no_method_suggested_traits::foo::PubPub`
error: aborting due to 6 previous errors
