
error: this operation will panic at runtime
  --> /Users/xavier/Code/rust/rust/src/test/ui/consts/const-eval/index-out-of-bounds-never-type.rs:10:61
   |
LL |     const VOID: ! = { let x = 0 * std::mem::size_of::<T>(); [][x] };
   |                                                             ^^^^^ index out of bounds: the len is 0 but the index is 0
   |
   = note: `#[deny(unconditional_panic)]` on by default
