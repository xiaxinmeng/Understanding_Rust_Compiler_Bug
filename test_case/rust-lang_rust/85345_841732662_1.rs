
error[E0255]: the name `Foo` is defined multiple times
  --> f21.rs:12:5
   |
1  | mod Foo {
   | ------- previous definition of the module `Foo` here
...
12 | use Foo::Foo;
   |     ^^^^^^^^ `Foo` reimported here
   |
   = note: `Foo` must be defined only once in the type namespace of this module
help: you can use `as` to change the binding name of the import
   |
12 | use Foo::Foo as OtherFoo;
   |     ^^^^^^^^^^^^^^^^^^^^
