
error[E0282]: type annotations needed
  --> src/main.rs:14:9
   |
13 |         let tmp = Foo::make_it();
   |             --- consider giving `tmp` a type
14 |         tmp.len();
   |         ^^^ cannot infer type
   |
   = note: type must be known at this point
