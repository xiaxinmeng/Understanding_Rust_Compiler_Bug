plain
   Compiling rand_xorshift v0.2.0
   Compiling rand_chacha v0.2.2
   Compiling rand v0.7.3
   Compiling alloc v0.0.0 (/checkout/library/alloc)
error[E0152]: found duplicate lang item `String`
    |
367 | pub struct String {
    | ^^^^^^^^^^^^^^^^^
    |
    |
    = note: the lang item is first defined in crate `alloc` (which `std` depends on)
    = note: first definition in `alloc` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-47a28fa4326b9c19.rlib, /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-47a28fa4326b9c19.rmeta
    = note: second definition in the local crate (`alloc`)
For more information about this error, try `rustc --explain E0152`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:17:53
