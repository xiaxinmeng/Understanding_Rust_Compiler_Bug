
---- src/lib.rs - foo (line 1) stdout ----
error[E0412]: cannot find type `Foo` in this scope
 --> src/lib.rs:7:17
  |
8 |     fn bar() -> Foo { Foo }
  |                 ^^^ not found in this scope

error[E0425]: cannot find value `Foo` in this scope
 --> src/lib.rs:7:23
  |
8 |     fn bar() -> Foo { Foo }
  |                       ^^^ not found in this scope
