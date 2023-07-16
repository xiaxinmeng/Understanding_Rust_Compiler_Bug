
error[E0425]: cannot find function `finish_grow_never_inline` in this scope
   --> library/alloc/src/raw_vec.rs:428:19
    |
428 |           let ptr = finish_grow_never_inline(new_layout, self.current_memory(), &mut self.alloc)?;
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `finish_grow_never_inlined`
...
502 | / fn finish_grow_never_inlined<A>(
503 | |     new_layout: Result<Layout, LayoutErr>,
504 | |     current_memory: Option<(NonNull<u8>, Layout)>,
505 | |     alloc: &mut A,
...   |
510 | |     finish_grow(new_layout, current_memory, alloc)
511 | | }
    | |_- similarly named function `finish_grow_never_inlined` defined here

error[E0412]: cannot find type `LayoutErr` in this scope
   --> library/alloc/src/raw_vec.rs:503:32
    |
503 |     new_layout: Result<Layout, LayoutErr>,
    |                                ^^^^^^^^^
    | 
   ::: /checkout/library/core/src/alloc/layout.rs:408:1
    |
408 | pub struct LayoutError {
    | ---------------------- similarly named struct `LayoutError` defined here
