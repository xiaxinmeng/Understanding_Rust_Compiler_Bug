rust
error[E0277]: the size for values of type `S` cannot be known at compilation time
  --> $DIR/constrain-suggest-ice.rs:3:36
   |
LL | struct Bug<S>{
   |            - this type parameter needs to be `std::marker::Sized`
LL |     A: [(); {
LL |         let x: [u8; Self::W] = [0; Self::W];
   |                                    ^^^^^^^ doesn't have a size known at compile-time
   |
note: required by a bound in `Bug`
  --> $DIR/constrain-suggest-ice.rs:1:12
   |
LL | struct Bug<S>{
   |            ^ required by this bound in `Bug`
help: consider relaxing the implicit `Sized` restriction
   |
LL | struct Bug<S: ?Sized>{
   |             ++++++++
