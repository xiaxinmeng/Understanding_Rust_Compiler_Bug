
error[E0277]: the trait bound `&str: std::convert::From<&std::string::String>` is not satisfied
 --> src/main.rs:6:15
  |
6 |     let str = <&str>::from(&string);
  |               ^^^^^^^^^^^^ the trait `std::convert::From<&std::string::String>` is not implemented for `&str`
  |
  = help: the following implementations were found:
            <&'t str as std::convert::From<regex::re_unicode::Match<'t>>>
  = note: required by `std::convert::From::from`
