plain
[RUSTC-TIMING] gimli test:false 6.316
[RUSTC-TIMING] object test:false 7.477
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-emscripten`

error: variants `Unregistered`, `Registered` and `RunningOrHasRun` are never constructed
    |
913 |     enum DtorState {
    |          --------- variants in this enum
914 |         Unregistered,
914 |         Unregistered,
    |         ^^^^^^^^^^^^
915 |         Registered,
    |         ^^^^^^^^^^
916 |         RunningOrHasRun,
    |
    |
    = note: `DtorState` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis
    = note: `-D dead-code` implied by `-D warnings`

error: fields `inner` and `dtor_state` are never read
    |
924 |     pub struct Key<T> {
    |                --- fields in this struct
...
...
933 |         inner: LazyKeyInner<T>,
    |         ^^^^^
...
937 |         dtor_state: Cell<DtorState>,

error: function `destroy_value` is never used
    --> library/std/src/thread/local.rs:1018:26
     |
     |
1018 |     unsafe extern "C" fn destroy_value<T>(ptr: *mut u8) {

error: associated function `new` is never used
   --> library/std/src/thread/local.rs:947:22
    |
    |
947 |         pub const fn new() -> Key<T> {

error: associated function `register_dtor` is never used
   --> library/std/src/thread/local.rs:954:23
    |
    |
954 |         pub unsafe fn register_dtor(a: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {

error: associated function `get` is never used
   --> library/std/src/thread/local.rs:960:23
    |
    |
960 |         pub unsafe fn get<F: FnOnce() -> T>(&self, init: F) -> Option<&'static T> {

error: associated function `try_initialize` is never used
   --> library/std/src/thread/local.rs:984:19
    |
    |
984 |         unsafe fn try_initialize<F: FnOnce() -> T>(&self, init: F) -> Option<&'static T> {

error: associated function `try_register_dtor` is never used
   --> library/std/src/thread/local.rs:997:19
    |
    |
997 |         unsafe fn try_register_dtor(&self) -> bool {
    |                   ^^^^^^^^^^^^^^^^^

error: function `register_dtor` is never used
  --> library/std/src/sys/unix/thread_local_dtor.rs:20:15
   |
20 | pub unsafe fn register_dtor(t: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {

[RUSTC-TIMING] std test:false 5.891
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to 9 previous errors; 1 warning emitted
