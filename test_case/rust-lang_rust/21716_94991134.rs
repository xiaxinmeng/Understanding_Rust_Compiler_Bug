 shell
$ rustc main.rs
main.rs:5:34: 5:48 error: ambiguous associated type; specify the type using the syntax `<Type as core::iter::Iterator>::Item` [E0223]
main.rs:5     fn next(&mut self) -> Option<Iterator::Item> {
                                           ^~~~~~~~~~~~~~
error: aborting due to previous error
