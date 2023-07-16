
$ rustc --lib 3366.rs
3366.rs:2:46: 2:56 error: instantiating a type parameter with an incompatible type `SomeParamName`, which does not fulfill `Send`
3366.rs:2 fn no_send<SomeParamName>(x: SomeParamName) { needs_send(x) }
                                                        ^~~~~~~~~~
error: aborting due to previous error
