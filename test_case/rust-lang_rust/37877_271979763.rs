
mkdir -p x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/
cp: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler-rt.a
rustc: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore
error[E0554]: #[feature] may not be used on the stable release channel
  --> src/libcore/lib.rs:70:1
   |
70 | #![feature(allow_internal_unstable)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: #[feature] may not be used on the stable release channel
  --> src/libcore/lib.rs:71:1
   |
71 | #![feature(asm)]
   | ^^^^^^^^^^^^^^^^
[snip]
