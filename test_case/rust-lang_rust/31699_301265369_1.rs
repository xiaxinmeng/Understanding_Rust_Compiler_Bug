
error[E0573]: expected type, found variant `function::Error::WrongType`
 --> test.rs:5:39
  |
5 | impl From<function::ArgumentType> for function::Error::WrongType {
  |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ not a type
  |
  = help: there is an enum variant `sxd_xpath::function::Error::WrongType`, did you mean to use `sxd_xpath::function::Error`?

error[E0573]: expected type, found variant `function::Error::WrongType`
 --> test.rs:6:50
  |
6 |     fn from(arg_type: function::ArgumentType) -> function::Error::WrongType {
  |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^ not a type
  |
  = help: there is an enum variant `sxd_xpath::function::Error::WrongType`, did you mean to use `sxd_xpath::function::Error`?

error: aborting due to 2 previous errors
