rust
15 |     };
   |     -- ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::option::Option<std::collections::binary_heap::PeekMut<'_, u32>>`
   |     |
   |     `heap_ref` dropped here while still borrowed
   |
   = note: The temporary is part of an expression at the end of a block. Consider forcing this temporary to be dropped sooner, before the block's local variables are dropped. For example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block.
