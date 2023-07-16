
error: Undefined Behavior: trying to retag from <3171> for Unique permission at alloc1601[0x0], but that tag does not exist in the borrow stack for this location
 --> demo.rs:9:14
  |
9 |         drop(new_box); // Call drop explicitly to make the error simpler
  |              ^^^^^^^
  |              |
  |              trying to retag from <3171> for Unique permission at alloc1601[0x0], but that tag does not exist in the borrow stack for this location
  |              this error occurs as part of retag at alloc1601[0x0..0x8]
  |
  = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
  = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
help: <3171> was created by a Unique retag at offsets [0x0..0x8]
 --> demo.rs:7:23
  |
7 |         let new_box = Box::from_raw(ptr as *mut usize);
  |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: <3171> was later invalidated at offsets [0x0..0x10] by a SharedReadOnly retag
 --> demo.rs:8:17
  |
8 |         let z = (*ptr).is_none();
  |                 ^^^^^^^^^^^^^^^^
  = note: BACKTRACE:
  = note: inside `main` at demo.rs:9:14
