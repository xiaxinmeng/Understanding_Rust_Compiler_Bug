rust
error[E0277]: can't compare `&str` with `str`
   --> src/libcore/../libcore/tests/iter.rs:822:19
    |
822 |     assert_eq!(it.next_if_eq("trillian"), None);
    |                   ^^^^^^^^^^ no implementation for `&str == str`
    |
    = help: the trait `std::cmp::PartialEq<str>` is not implemented for `&str`
