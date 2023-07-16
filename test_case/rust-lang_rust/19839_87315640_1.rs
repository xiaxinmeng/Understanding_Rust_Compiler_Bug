
> rustc --version
rustc 1.0.0-nightly (199bdcfef 2015-03-26) (built 2015-03-27)
> rustc foo.rs
foo.rs:8:9: 8:16 error: type `Foo<'_, Bar>` does not implement any method in scope named `clone`
foo.rs:8     foo.clone();
                 ^~~~~~~
foo.rs:8:16: 8:16 help: methods from traits can only be called if the trait is implemented and in scope; the following trait defines a method `clone`, perhaps you need to implement it:
foo.rs:8:16: 8:16 help: candidate #1: `core::clone::Clone`
