
error[E0277]: a value of type `i32` cannot be made by summing an iterator over elements of type `()`
    --> src/test/ui/iterators/invalid-iterator-chain.rs:7:20
     |
7    |     println!("{}", scores.sum::<i32>());
     |                    ^^^^^^ --- required by a bound introduced by this call
     |                    |
     |                    value of type `i32` cannot be made by summing a `std::iter::Iterator<Item=()>`
     |
     = note: the trait bound `i32: Sum<()>` is not satisfied
     = help: the following other types implement trait `Sum<A>`:
               <i32 as Sum<&'a i32>>
               <i32 as Sum>
note: the expression is of type `Map<std::slice::Iter<'_, ({integer}, {integer})>, [closure@src/test/ui/iterators/invalid-iterator-chain.rs:4:14: 4:22]>`
    --> src/test/ui/iterators/invalid-iterator-chain.rs:7:20
     |
2    |       let scores = vec![(0, 0)]
     |                    ------------ this expression has type `Vec<({integer}, {integer})>`
3    |           .iter()
     |            ------ associated type `std::iter::Iterator::Item` is `&({integer}, {integer})` here
4    |           .map(|(a, b)| {
     |  __________-
5    | |             a + b;
6    | |         });
     | |__________- associated type `std::iter::Iterator::Item` is `()` here
7    |       println!("{}", scores.sum::<i32>());
     |                      ^^^^^^
note: required by a bound in `std::iter::Iterator::sum`
    --> /Users/ekuber/workspace/rust/library/core/src/iter/traits/iterator.rs:3379:12
     |
3379 |         S: Sum<Self::Item>,
     |            ^^^^^^^^^^^^^^^ required by this bound in `std::iter::Iterator::sum`
