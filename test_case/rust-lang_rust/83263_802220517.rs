
error: expected identifier, found reserved identifier `_`
  --> src/main.rs:10:42
   |
10 |     if let Some(TopException::HardFault {_}) = top_exception {
   |                                          ^ expected identifier, found reserved identifier

error[E0026]: variant `TopException::HardFault` does not have a field named `_`
  --> src/main.rs:10:42
   |
10 |     if let Some(TopException::HardFault {_}) = top_exception {
   |                                          ^ variant `TopException::HardFault` does not have this field

error[E0027]: pattern does not mention field `stack_overflow`
  --> src/main.rs:10:17
   |
10 |     if let Some(TopException::HardFault {_}) = top_exception {
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `stack_overflow`
   |
help: include the missing field in the pattern
   |
10 |     if let Some(TopException::HardFault {_, stack_overflow }) = top_exception {
   |                                           ^^^^^^^^^^^^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
   |
10 |     if let Some(TopException::HardFault {_, .. }) = top_exception {
