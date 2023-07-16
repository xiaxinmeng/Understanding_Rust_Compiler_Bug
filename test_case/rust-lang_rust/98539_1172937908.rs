
error[E0599]: the method `get` exists for struct `Victim<'_, Self>`, but its trait bounds were not satisfied
  --> src/test/ui/trait-bounds/impl-derived-implicit-sized-bound.rs:31:19
   |
1  | / struct Victim<'a, T: Perpetrator + ?Sized>
2  | | where
3  | |   Self: Sized
4  | | {
5  | |   value: u8,
6  | |   perp: &'a T,
7  | | }
   | | -
   | | |
   | |_method `get` not found for this
   |   doesn't satisfy `Victim<'_, Self>: VictimTrait`
...
31 |       self.getter().get();
   |                     ^^^ method cannot be called on `Victim<'_, Self>` due to unsatisfied trait bounds
   |
note: trait bound `Self: Sized` was not satisfied
  --> src/test/ui/trait-bounds/impl-derived-implicit-sized-bound.rs:15:10
   |
15 | impl<'a, T: Perpetrator /*+ ?Sized*/> VictimTrait for Victim<'a, T> {
   |          ^                            -----------     -------------
   |          |
   |          unsatisfied trait bound introduced here
help: consider relaxing the type parameter's implicit `?Sized` bound
   |
15 | impl<'a, T: ?Sized + Perpetrator /*+ ?Sized*/> VictimTrait for Victim<'a, T> {
   |             ++++++++
help: consider restricting the type parameter to satisfy the trait bound
   |
3  |   Self: Sized, Self: Sized
   |              +++++++++++++

error: aborting due to previous error
