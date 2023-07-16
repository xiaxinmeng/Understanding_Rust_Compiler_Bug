 rust
$ rustc --cfg error 1946.rs
1946.rs:27:10: 27:15 error: multiple applicable methods in scope [E0034]
1946.rs:27     Quux.baz();
                    ^~~~~
1946.rs:23:1: 23:26 note: candidate #1 is defined in an impl of the trait `foo::Foo` for the type `Quux`
1946.rs:23 impl foo::Foo for Quux {}
           ^~~~~~~~~~~~~~~~~~~~~~~~~
1946.rs:24:1: 24:26 note: candidate #2 is defined in an impl of the trait `bar::Bar` for the type `Quux`
1946.rs:24 impl bar::Bar for Quux {}
           ^~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
