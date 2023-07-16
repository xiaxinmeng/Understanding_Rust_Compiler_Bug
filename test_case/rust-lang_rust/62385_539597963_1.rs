
error[E0277]: the trait bound `Dummy: std::convert::From<fn(u32) -> std::string::String {foo}>` is not satisfied
  --> src/main.rs:10:5
   |
10 |     Dummy::from(foo);
   |     ^^^^^^^^^^^ the trait `std::convert::From<fn(u32) -> std::string::String {foo}>` is not implemented for `Dummy`
   |
   = help: the following implementations were found:
             <Dummy as std::convert::From<fn(u32) -> std::string::String>>
   = note: required by `std::convert::From::from`
