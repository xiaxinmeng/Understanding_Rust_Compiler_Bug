plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `alignment::Alignment::new_unchecked` is not yet stable as a const fn
  --> library/core/src/alloc/layout.rs:74:54
   |
74 |         Layout::from_size_valid_align(size, unsafe { ValidAlign::new_unchecked(align) })
   |
   = help: const-stable functions can only call other const-stable functions

error: `alignment::Alignment::new_unchecked` is not yet stable as a const fn
error: `alignment::Alignment::new_unchecked` is not yet stable as a const fn
   --> library/core/src/alloc/layout.rs:119:40
    |
119 |         unsafe { Layout { size, align: ValidAlign::new_unchecked(align) } }
    |
    = help: const-stable functions can only call other const-stable functions


error: `alignment::Alignment::as_usize` is not yet stable as a const fn
    |
    |
138 |         self.align.as_usize()
    |
    = help: const-stable functions can only call other const-stable functions

error: could not compile `core` due to 3 previous errors
