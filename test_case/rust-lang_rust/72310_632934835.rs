
error[E0277]: the trait bound `std::string::String: std::borrow::Borrow<&str>` is not satisfied
   --> src/libcore/../libcore/tests/iter.rs:835:19
    |
835 |     assert_eq!(it.next_if_eq("Ludicrous"), Some("Ludicrous".into()));
    |                   ^^^^^^^^^^ the trait `std::borrow::Borrow<&str>` is not implemented for `std::string::String`
    |
    = help: the following implementations were found:
              <std::string::String as std::borrow::Borrow<str>>
