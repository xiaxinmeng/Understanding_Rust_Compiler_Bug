
error[E0631]: type mismatch in closure arguments
   --> src/main.rs:97:10
    |
97  |         .filter(|tower: &mut bevy::prelude::Mut<'_, Tower>| {
    |          ^^^^^^ ------------------------------------------- found signature defined here
    |          |
    |          expected due to this
    |
    = note: expected closure signature `for<'a> fn(&'a Mut<'_, Tower>) -> _`
               found closure signature `for<'a, 'b> fn(&'a mut Mut<'b, Tower>) -> _`
note: required by a bound in `std::iter::Iterator::filter`
   --> /home/matt/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:899:12
    |
899 |         P: FnMut(&Self::Item) -> bool,
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `std::iter::Iterator::filter`

error[E0599]: the method `for_each` exists for struct `Filter<QueryIter<'_, '_, &mut Tower, ()>, [closure@src/main.rs:97:17: 97:60]>`, but its trait bounds were not satisfied
   --> src/main.rs:101:10
    |
97  |         .filter(|tower: &mut bevy::prelude::Mut<'_, Tower>| {
    |                 -------------------------------------------
    |                 |
    |                 doesn't satisfy `<_ as FnOnce<(&Mut<'_, Tower>,)>>::Output = bool`
    |                 doesn't satisfy `_: FnMut<(&Mut<'_, Tower>,)>`
...
101 |         .for_each(|tower| {
    |          ^^^^^^^^ method cannot be called on `Filter<QueryIter<'_, '_, &mut Tower, ()>, [closure@src/main.rs:97:17: 97:60]>` due to unsatisfied trait bounds
    |
   ::: /home/matt/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/filter.rs:15:1
    |
15  | pub struct Filter<I, P> {
    | ----------------------- doesn't satisfy `_: Iterator`
    |
    = note: the following trait bounds were not satisfied:
            `<[closure@src/main.rs:97:17: 97:60] as FnOnce<(&Mut<'_, Tower>,)>>::Output = bool`
            which is required by `Filter<QueryIter<'_, '_, &mut Tower, ()>, [closure@src/main.rs:97:17: 97:60]>: Iterator`
            `[closure@src/main.rs:97:17: 97:60]: FnMut<(&Mut<'_, Tower>,)>`
            which is required by `Filter<QueryIter<'_, '_, &mut Tower, ()>, [closure@src/main.rs:97:17: 97:60]>: Iterator`
            `Filter<QueryIter<'_, '_, &mut Tower, ()>, [closure@src/main.rs:97:17: 97:60]>: Iterator`
            which is required by `&mut Filter<QueryIter<'_, '_, &mut Tower, ()>, [closure@src/main.rs:97:17: 97:60]>: Iterator`

Some errors have detailed explanations: E0599, E0631.
For more information about an error, try `rustc --explain E0599`.
error: could not compile `tutorial` due to 2 previous errors
