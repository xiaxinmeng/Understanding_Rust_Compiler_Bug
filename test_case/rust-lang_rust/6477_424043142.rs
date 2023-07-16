text
error[E0412]: cannot find type `Foo` in this scope
 --> .\foo.rs:8:10
  |
8 |      x : Foo
  |          ^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
  |
6 |    use foo::Foo;
  |
