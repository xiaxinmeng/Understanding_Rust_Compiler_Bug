
error[E0425]: cannot find value `Foo` in module `m` (BASE 2)
 --> test.rs:8:15
  |
8 |     let foo = m::Foo;
  |               ^^^^^^ (LABEL 2)
  |
  = help: possible candidate is found in another module, you can import it into scope:
  = help:   `use m::n::Foo;`
