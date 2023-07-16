rust
extern crate sxd_xpath;

use sxd_xpath::function;

impl From<function::ArgumentType> for function::Error::WrongType {
    fn from(arg_type: function::ArgumentType) -> function::Error::WrongType {

    }
}
