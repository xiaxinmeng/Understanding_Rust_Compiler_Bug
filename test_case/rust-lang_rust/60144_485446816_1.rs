
error[E0277]: the trait bound `Foo: Bar<&std::vec::Vec<u8>>` is not satisfied
 --> src/main.rs:3:9
  |
3 |     Foo.bar(&baz); // Causes error
  |         ^^^ the trait `Bar<&std::vec::Vec<u8>>` is not implemented for `Foo`
  |
  = help: the following implementations were found:
            <Foo as Bar<&[u8]>>
            <Foo as Bar<()>>
