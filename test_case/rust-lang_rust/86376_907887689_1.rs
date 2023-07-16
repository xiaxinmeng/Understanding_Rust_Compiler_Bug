
warning: `#[no_mangle]` has no effect on foreign statics
  --> $DIR/extern-no-mangle.rs:11:5
   |
LL |     #[no_mangle]
   |     ^^^^^^^^^^^^ help: remove this attribute
...
LL |     pub static FOO: u8;
   |     ------------------- foreign static
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: symbol names in extern blocks are not mangled by default
