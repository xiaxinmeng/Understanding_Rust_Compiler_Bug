
src/lib.rs:11:1: 11:23 error: conflicting implementations for trait `a::Baz` [E0119]
src/lib.rs:11 impl Baz<i32> for S {}
              ^~~~~~~~~~~~~~~~~~~~~~
src/lib.rs:12:1: 12:38 note: note conflicting implementation here
src/lib.rs:12 impl Baz<<a::T as Foo>::Bar> for S {}
              ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
