
error[E0425]: cannot find function `function_that_doesnt_exist` in this scope
  --> src/main.rs:14:13
   |
14 |     let s = function_that_doesnt_exist();
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope

error[E0308]: mismatched types
  --> src/main.rs:16:11
   |
16 |     test2(s);
   |           ^
   |           |
   |           expected struct `std::string::String`, found `str`
   |           help: try using a conversion method: `s.to_string()`
