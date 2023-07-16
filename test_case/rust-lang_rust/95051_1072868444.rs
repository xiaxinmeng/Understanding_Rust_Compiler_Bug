plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0308]: mismatched types
   --> library/alloc/src/raw_vec/tests.rs:107:26
    |
107 |     let v: RawVec<ZST> = RawVec::allocate_in(0, AllocInit::Uninitialized, Global);
    |            -----------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `raw_vec::RawVec`, found enum `Result`
    |            expected due to this
    |
    |
    = note: expected struct `raw_vec::RawVec<ZST>`
                 found enum `Result<raw_vec::RawVec<_>, _>`
error[E0308]: mismatched types
   --> library/alloc/src/raw_vec/tests.rs:110:26
    |
    |
110 |     let v: RawVec<ZST> = RawVec::allocate_in(100, AllocInit::Uninitialized, Global);
    |            -----------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `raw_vec::RawVec`, found enum `Result`
    |            expected due to this
    |
    |
    = note: expected struct `raw_vec::RawVec<ZST>`
                 found enum `Result<raw_vec::RawVec<_>, _>`
error[E0308]: mismatched types
   --> library/alloc/src/raw_vec/tests.rs:113:30
    |
    |
113 |     let mut v: RawVec<ZST> = RawVec::allocate_in(usize::MAX, AllocInit::Uninitialized, Global);
    |                -----------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `raw_vec::RawVec`, found enum `Result`
    |                expected due to this
    |
    |
    = note: expected struct `raw_vec::RawVec<ZST>`
                 found enum `Result<raw_vec::RawVec<_>, _>`

error[E0599]: no method named `try_reserve` found for struct `raw_vec::RawVec` in the current scope
   --> library/alloc/src/raw_vec/tests.rs:130:18
    |
130 |     assert_eq!(v.try_reserve(100, usize::MAX - 100), Ok(()));
    |                  ^^^^^^^^^^^ method not found in `raw_vec::RawVec<ZST>`
   ::: library/alloc/src/raw_vec.rs:49:1
    |
    |
49  | pub(crate) struct RawVec<T, A: Allocator = Global> {
    | -------------------------------------------------- method `try_reserve` not found for this

error[E0599]: no method named `try_reserve` found for struct `raw_vec::RawVec` in the current scope
   --> library/alloc/src/raw_vec/tests.rs:131:18
    |
131 |     assert_eq!(v.try_reserve(101, usize::MAX - 100), cap_err);
    |                  ^^^^^^^^^^^ method not found in `raw_vec::RawVec<ZST>`
   ::: library/alloc/src/raw_vec.rs:49:1
    |
    |
49  | pub(crate) struct RawVec<T, A: Allocator = Global> {
    | -------------------------------------------------- method `try_reserve` not found for this
Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `alloc` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
