
rustc 1.16.0 (30cf806ef 2017-03-10)
error[E0275]: overflow evaluating the requirement `Foo: std::marker::Send`
  --> <anon>:21:5
   |
21 |     f(Foo(Arc::new(Bar::B(None))));
   |     ^
   |
   = note: consider adding a `#![recursion_limit="128"]` attribute to your crate
   = note: required because it appears within the type `std::option::Option<Foo>`
   = note: required because it appears within the type `Bar`
   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<Bar>`
   = note: required because it appears within the type `Foo`
   = note: required because it appears within the type `std::option::Option<Foo>`
   = note: required because it appears within the type `Bar`
   = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<Bar>`
   = note: required because it appears within the type `Foo`
...
