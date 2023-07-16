
error[E0277]: the trait bound `i32: Sum<()>` is not satisfied
    --> f52.rs:6:20
     |
6    |     println!("{}", scores.sum::<i32>());
     |                    ^^^^^^ --- required by a bound introduced by this call
     |                    |
     |                    the trait `Sum<()>` is not implemented for `i32`
     |
     = help: the following other types implement trait `Sum<A>`:
               <f32 as Sum<&'a f32>>
               <f32 as Sum>
               <f64 as Sum<&'a f64>>
               <f64 as Sum>
               <i128 as Sum<&'a i128>>
               <i128 as Sum>
               <i16 as Sum<&'a i16>>
               <i16 as Sum>
             and 20 others
note: required by a bound in `std::iter::Iterator::sum`
    --> /Users/ekuber/workspace/rust/library/core/src/iter/traits/iterator.rs:3379:12
     |
3379 |         S: Sum<Self::Item>,
     |            ^^^^^^^^^^^^^^^ required by this bound in `std::iter::Iterator::sum`
