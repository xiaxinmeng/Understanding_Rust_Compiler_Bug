
src/librustdoc/test.rs:108:47: 108:51 error: mismatched types [E0308]
src/librustdoc/test.rs:108     let map = hir_map::map_crate(&mut forest, defs);
                                                                         ^~~~
src/librustdoc/test.rs:108:47: 108:51 help: run `rustc --explain E0308` to see a detailed explanation
src/librustdoc/test.rs:108:47: 108:51 note: expected type `rustc::hir::map::Definitions`
src/librustdoc/test.rs:108:47: 108:51 note:    found type `&std::cell::RefCell<rustc::hir::map::Definitions>`
error: aborting due to previous error
