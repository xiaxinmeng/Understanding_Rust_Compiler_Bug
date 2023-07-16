 shell
$ rustc main.rs
main.rs:7:45: 7:59 error: ambiguous associated type; specify the type using the syntax `<Type as serialize::serialize::Decoder>::Error` [E0223]
main.rs:7     fn decode<D>(s: &mut D) -> Result<Self, Decoder::Error>
                                                      ^~~~~~~~~~~~~~
error: aborting due to previous error
