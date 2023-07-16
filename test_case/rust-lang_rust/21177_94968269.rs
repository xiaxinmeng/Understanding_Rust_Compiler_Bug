 shell
$ rustc main.rs
main.rs:6:21: 6:25 error: unsupported cyclic reference between types/traits detected
main.rs:6 fn foo<T: Trait<A = T::B>>() { }
                              ^~~~
note: the cycle begins when computing the bounds for type parameter `T`...
note: ...which then again requires computing the bounds for type parameter `T`, completing the cycle.
error: aborting due to previous error
