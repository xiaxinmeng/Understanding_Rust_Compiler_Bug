
error[[E0038]](https://doc.rust-lang.org/nightly/error-index.html#E0038): the trait `ObjectSafe` cannot be made into an object
 --> src/main.rs:9:26
  |
9 | fn takes_object_safe(os: dyn ObjectSafe) {}
  |                          ^^^^^^^^^^^^^^ `ObjectSafe` cannot be made into an object
  |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
 --> src/main.rs:4:11
  |
4 |     const Const: i32;
  |           ^^^^^ ...because it contains this associated `const`
...
7 | trait ObjectSafe: NotObjectSafe {}
  |       ---------- this trait cannot be made into an object...
  = help: consider moving `Const` to another trait
