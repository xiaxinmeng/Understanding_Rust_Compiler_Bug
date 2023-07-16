
error[[E0080]](https://doc.rust-lang.org/nightly/error-index.html#E0080): it is undefined behavior to use this value
  --> src/main.rs:26:1
   |
26 | static mut GLOBAL_LAZY_MUT: MyStruct = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .some_bits.<enum-tag>: encountered 0xa5, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
               a5                                              │ .
           }
