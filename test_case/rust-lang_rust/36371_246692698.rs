
error[E0046]: not all trait items implemented, missing: `foo`
  --> src/test/compile-fail/E0046.rs:17:1
   |
12 |     fn foo();
   |     -------- `foo` from trait
...
17 | impl Foo for Bar {}
   | ^^^^^^^^^^^^^^^^^^^ expected `foo` in implementation
