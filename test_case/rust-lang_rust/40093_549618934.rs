
error[E0277]: the trait bound `<T as Test>::Bundle: std::convert::From<std::string::String>` is not satisfied
  --> src/lib.rs:10:1
   |
1  |   pub trait Test {
   |   -------------- required by `Test`
...
10 | / fn fails<T>() 
11 | |     where T: Test<Item = String>
   | |                                 - help: consider further restricting the associated type: `, <T as Test>::Bundle: std::convert::From<std::string::String>`
12 | | { } 
   | |___^ the trait `std::convert::From<std::string::String>` is not implemented for `<T as Test>::Bundle`
