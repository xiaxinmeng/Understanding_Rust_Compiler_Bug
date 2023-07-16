plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: strict provenance disallows casting integer `usize` to pointer `*const T`
    |
102 |         bits as Self
    |         ^^^^^^^^^^^^
    |
    |
    = note: `-D fuzzy-provenance-cast` implied by `-D warnings`
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address

error: strict provenance disallows casting pointer `*const T` to integer `usize`
    |
119 |         self as usize
    |         ^^^^^^^^^^^^^
    |
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.addr()` to obtain the address of a pointer

error: strict provenance disallows casting integer `usize` to pointer `*mut T`
    |
105 |         bits as Self
    |         ^^^^^^^^^^^^
    |
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address

error: strict provenance disallows casting pointer `*mut T` to integer `usize`
    |
122 |         self as usize
    |         ^^^^^^^^^^^^^
    |
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.addr()` to obtain the address of a pointer

error: strict provenance disallows casting integer `usize` to pointer `*mut T`
    |
367 |     addr as *mut T
    |     ^^^^^^^^^^^^^^
    |
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address

error: strict provenance disallows casting integer `usize` to pointer `*const T`
    |
382 |     addr as *const T
    |     ^^^^^^^^^^^^^^^^
    |
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address

error: strict provenance disallows casting integer `usize` to pointer `*mut T`
    |
397 |     addr as *mut T
    |     ^^^^^^^^^^^^^^
    |
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
    = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address

error: strict provenance disallows casting pointer `*mut u128` to integer `usize`
  --> library/core/src/../../stdarch/crates/core_arch/src/x86_64/cmpxchg16b.rs:54:19
   |
54 |     debug_assert!(dst as usize % 16 == 0);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
   = note: for more information, see issue #9999999 <https://github.com/rust-lang/rust/issues/9999999>
   = help: use `.addr()` to obtain the address of a pointer
error: could not compile `core` due to 8 previous errors
Build completed unsuccessfully in 0:03:45
