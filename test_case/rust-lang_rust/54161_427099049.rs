
error[E0599]: no method named `hello` found for type `MyStruct` in the current scope
  --> src/main.rs:12:7
   |
1  | struct MyStruct;
   | ---------------- method `hello` not found for this
...
12 |     a.hello();
   |       ^^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
   |
1  | use some_mod::MyTrait;
   |
