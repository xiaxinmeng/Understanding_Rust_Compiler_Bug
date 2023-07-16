
warning: `#[no_mangle]` should not be applied to a foreign static
  --> $DIR/extern-no-mangle.rs:11:5
   |
LL |     #[no_mangle]
   |     ^^^^^^^^^^^^ help: remove this attribute
...
LL |     pub static FOO: u8;
   |     ------------------- foreign static
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: foreign symbol names are always preserved and the `#[no_mangle]` attribute may have additional undesired exporting effects.
