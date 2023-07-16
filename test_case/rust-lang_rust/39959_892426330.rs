
error[E0275]: overflow evaluating the requirement `Ser<'_, Option<_>>: Serialize`
  --> src/main.rs:18:5
   |
18 |     Ser::new(value);
   |     ^^^^^^^^
   |
   = help: consider adding a `#![recursion_limit="10"]` attribute to your crate (`playground`)
note: required because of the requirements on the impl of `Serialize` for `Ser<'_, Option<Option<_>>>`
  --> src/main.rs:15:13
   |
15 | impl<'a, T> Serialize for Ser<'a, Option<T>> where Ser<'a, T>: Serialize {}
   |             ^^^^^^^^^     ^^^^^^^^^^^^^^^^^^
   = note: 4 redundant requirements hidden
   = note: required because of the requirements on the impl of `Serialize` for `Ser<Option<Option<Option<Option<Option<Option<_>>>>>>>`
