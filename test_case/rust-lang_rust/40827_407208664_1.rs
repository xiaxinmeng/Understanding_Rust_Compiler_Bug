text
error[E0275]: overflow evaluating the requirement `std::sync::Arc<List>: std::marker::Send`
  --> src/lib.rs:12:24
   |
12 |     let _: Box<Send> = Box::new(List { value: Rc::new(4), next: None });
   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
   = note: required because it appears within the type `std::option::Option<std::sync::Arc<List>>`
   = note: required because it appears within the type `List`
   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<List>`
   = note: required because it appears within the type `std::option::Option<std::sync::Arc<List>>`
