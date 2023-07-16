
error[E0038]: the trait `SuperTrait` cannot be made into an object
  --> src/test/ui/generic-associated-types/issue-76535.rs:35:15
   |
35 | fn test(arg : Box<dyn for<'a> SuperTrait<SubType<'a> = SubStruct<'a>>>) -> u32 {
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `SuperTrait` cannot be made into an object
   |
   = help: consider moving `get_sub` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> src/test/ui/generic-associated-types/issue-76535.rs:8:37
   |
5  | pub trait SuperTrait {
   |           ---------- this trait cannot be made into an object...
...
8  |     fn get_sub<'a>(&'a mut self) -> Self::SubType<'a>;
   |                                     ^^^^^^^^^^^^^^^^^ ...because method `get_sub` references the `Self` type in its return type
