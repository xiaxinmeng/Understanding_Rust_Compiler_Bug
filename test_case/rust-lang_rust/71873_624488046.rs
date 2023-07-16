
# ./x.py test --bless --stage 1
[...]
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling getrandom v0.1.14
   Compiling ppv-lite86 v0.2.6
   Compiling c2-chacha v0.2.3
   Compiling rand_core v0.5.1
   Compiling rand_xorshift v0.2.0
   Compiling rand_chacha v0.2.1
   Compiling rand v0.7.3
   Compiling alloc v0.0.0 (/home/tim/Code/rust/src/liballoc)
error[E0308]: mismatched types
   --> src/liballoc/vec.rs:685:13
    |
679 |     pub fn into_boxed_slice(mut self) -> Box<[T]> {
    |                                          -------- expected `std::boxed::Box<[T]>` because of return type
...
685 |             buf.into_box(len).assume_init()
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::alloc::Global`, found struct `alloc::Global`
    |
    = note: expected struct `std::boxed::Box<_, std::alloc::Global>`
               found struct `std::boxed::Box<_, alloc::Global>`
