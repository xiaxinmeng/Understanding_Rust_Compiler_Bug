
$ rustc other.rs
$ rustc main.rs -L.
main.rs:7:12: 7:29 error: wrong number of lifetime parameters: expected 0 but found 1
main.rs:7 impl<'v, D: other::Decoder<'v>> other::Decodable<'v, D> for S {
                      ^~~~~~~~~~~~~~~~~
main.rs:7:32: 7:55 error: wrong number of lifetime parameters: expected 0 but found 1
main.rs:7 impl<'v, D: other::Decoder<'v>> other::Decodable<'v, D> for S {
                                          ^~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
