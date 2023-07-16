rust
   Compiling playground v0.0.1 (/playground)
error[E0599]: no method named `bar` found for slice `[u8]` in the current scope
  --> src/lib.rs:14:24
   |
14 |     unsafe { (*t).data.bar() }
   |                        ^^^ method not found in `[u8]`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Foo` defines an item `bar`, perhaps you need to implement it
  --> src/lib.rs:1:1
   |
1  | trait Foo {
   | ^^^^^^^^^
