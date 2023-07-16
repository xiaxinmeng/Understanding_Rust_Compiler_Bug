
error[[E0277]](https://doc.rust-lang.org/stable/error_codes/E0277.html): the size for values of type `A` cannot be known at compilation time
 --> src/main.rs:2:19
  |
1 |   struct B<A: Sized = [(); {
  |  __________-
2 | |     let x = [0u8; !0usize];
  | |                   ^^^^^^^ doesn't have a size known at compile-time
3 | |     1
4 | | }]> {
  | |__- this type parameter needs to be `std::marker::Sized`
  |
note: required by a bound in `B`
 --> src/main.rs:1:10
  |
1 |   struct B<A: Sized = [(); {
  |  __________^
2 | |     let x = [0u8; !0usize];
3 | |     1
4 | | }]> {
  | |__^ required by this bound in `B`

For more information about this error, try `rustc --explain E0277`.
