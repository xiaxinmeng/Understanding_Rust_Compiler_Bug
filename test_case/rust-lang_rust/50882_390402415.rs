plain
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:08]    Compiling libc v0.2.40
[01:05:09]    Compiling rand v0.4.2
[01:05:11]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:05:14] error[E0624]: method `into_raw_and_alloc` is private
[01:05:14]     |
[01:05:14]     |
[01:05:14] 195 |             let (ptr, a) = Box::into_raw_and_alloc(slice);
[01:05:14] 
[01:05:14] error[E0308]: mismatched types
[01:05:14]    --> liballoc/vec.rs:645:13
[01:05:14]     |
[01:05:14]     |
[01:05:14] 640 |     pub fn into_boxed_slice(mut self) -> Box<[T]> {
[01:05:14]     |                                          -------- expected `std::boxed::Box<[T]>` because of return type
[01:05:14] ...
[01:05:14] 645 |             buf.into_box()
[01:05:14]     |             ^^^^^^^^^^^^^^ expected struct `std::heap::Global`, found struct `alloc::Global`
[01:05:14]     |
[01:05:14]     = note: expected type `std::boxed::Box<_, std::heap::Global>`
[01:05:14]                found type `std::boxed::Box<_, alloc::Global>`
[01:05:16] error: aborting due to 2 previous errors
[01:05:16] 
[01:05:16] Some errors occurred: E0308, E0624.
[01:05:16] For more information about an error, try `rustc --explain E0308`.
---
103868 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
103864 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
103604 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
103220 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt
103216 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt/s-f16j5w4jts-1guxjqd-1fl9pp8y1eh0p
91808 ./obj/build/x86_64-unknown-linux-gnu/stage1
91784 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
89792 ./src/llvm/test/CodeGen
89200 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
