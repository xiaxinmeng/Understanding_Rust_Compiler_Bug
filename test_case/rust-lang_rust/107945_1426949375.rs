text
warning: the type `std::arch::x86_64::__m128i` does not permit being left uninitialized
 --> src/lib.rs:5:9
  |
5 |         std::mem::MaybeUninit::uninit().assume_init()
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |         |
  |         this code causes undefined behavior when executed
  |         help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
  |
  = note: integers must be initialized
