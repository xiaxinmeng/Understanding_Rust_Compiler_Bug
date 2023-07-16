
error[E0277]: a value of type `i32` cannot be made by summing an iterator over elements of type `()`
  --> $DIR/invalid-iterator-chain.rs:7:27
   |
LL |     println!("{}", scores.sum::<i32>());
   |                           ^^^ value of type `i32` cannot be made by summing a `std::iter::Iterator<Item=()>`
   |
   = help: the trait `Sum<()>` is not implemented for `i32`
   = help: the following other types implement trait `Sum<A>`:
             <i32 as Sum<&'a i32>>
             <i32 as Sum>
note: the method call chain might not have had the expected associated types
  --> $DIR/invalid-iterator-chain.rs:4:10
   |
LL |       let scores = vec![(0, 0)]
   |                    ------------ this expression has type `Vec<({integer}, {integer})>`
LL |           .iter()
   |            ------ `Iterator::Item` is `&({integer}, {integer})` here
LL |           .map(|(a, b)| {
   |  __________^
LL | |             a + b;
LL | |         });
   | |__________^ `Iterator::Item` changed to `()` here
note: required by a bound in `std::iter::Iterator::sum`
  --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
   |
LL |         S: Sum<Self::Item>,
   |            ^^^^^^^^^^^^^^^ required by this bound in `Iterator::sum`
