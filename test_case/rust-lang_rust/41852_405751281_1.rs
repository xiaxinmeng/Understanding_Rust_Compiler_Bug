
error[E0599]: no function or associated item named `bat` found for type `Foo` in the current scope
 --> src/main.rs:8:5
  |
1 | struct Foo;
  | ----------- function or associated item `bat` not found for this
...
8 |     Foo::bat(());
  |     ^^^^^^^^ function or associated item not found in `Foo`
  |
  = note: the method `bat` exists but the following trait bounds were not satisfied:
          `Foo : Bat`
          `&Foo : Bat`
          `&mut Foo : Bat`
  = help: items from traits can only be used if the trait is implemented and in scope
  = note: the following trait defines an item `bat`, perhaps you need to implement it:
          candidate #1: `Bat`
