
error[E0599]: no function or associated item named `foo` found for type `submodule::A` in the current scope
  --> src/main.rs:16:14
   |
6  |     pub struct A {}
   |     ------------ function or associated item `foo` not found for this
...
16 |     MyThing::foo();
   |     ---------^^^
   |     |
   |     function or associated item not found in `submodule::A`
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
   |
11 | use crate::submodule::MyThing;
   |
