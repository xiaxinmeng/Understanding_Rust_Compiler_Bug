
error[E0277]: can't compare `str` with `std::string::String`
   --> compiler/rustc_macros/src/symbols.rs:101:20
    |
101 |             if str < prev_str {
    |                    ^ no implementation for `str < std::string::String` and `str > std::string::String`
    |
    = help: the trait `std::cmp::PartialOrd<std::string::String>` is not implemented for `str`
    = note: required because of the requirements on the impl of `std::cmp::PartialOrd<&std::string::String>` for `&str`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_macros`.
