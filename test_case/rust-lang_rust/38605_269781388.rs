rust
error[E0308]: inappropriate pattern for type
 --> file.rs:3:8
  |
3 | fn foo(&foo: Foo) {}
  |        ^^^^ `&` pattern cannot be used to match a value of type `Foo`
  |
  = help: use `foo: &Foo` to declare an argument `foo` of type `&Foo`
