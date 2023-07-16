
warning: the type `Tag` does not permit being left uninitialized
  --> src/main.rs:23:33
   |
23 |     let mut tag: Tag = unsafe { mem::uninitialized() };
   |                                 ^^^^^^^^^^^^^^^^^^^^
   |                                 |
   |                                 this code causes undefined behavior when executed
   |                                 help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: `#[warn(invalid_value)]` on by default
note: enums have to be initialized to a variant
  --> src/main.rs:4:1
   |
4  | / pub enum Tag {
5  | |     Undefined = 0,
6  | |     Null = 1,
7  | |     I32 = 2,
...  |
19 | |     Symbol = 15
20 | | }
   | |_^
