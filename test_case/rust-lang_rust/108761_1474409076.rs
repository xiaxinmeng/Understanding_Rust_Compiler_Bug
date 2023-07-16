plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
    Checking addr2line v0.17.0
error[E0428]: the name `DTORS` is defined multiple times
  --> /checkout/library/std/src/sys/unix/thread_local_dtor.rs:72:5
   |
62 |     static mut DTORS: PlVec<(*mut u8, unsafe extern "C" fn(*mut u8))> = PlVec::new();
   |     --------------------------------------------------------------------------------- previous definition of the value `DTORS` here
...
72 |     static DTORS: Cell<*mut List> = Cell::new(ptr::null_mut());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `DTORS` redefined here
   |
   = note: `DTORS` must be defined only once in the value namespace of this block
error[E0061]: this method takes 1 argument but 0 arguments were supplied
   --> /checkout/library/std/src/sys/unix/thread_local_dtor.rs:73:14
    |
    |
73  |     if DTORS.get().is_null() {
    |              ^^^-- an argument is missing
note: method defined here
   --> /checkout/library/core/src/slice/mod.rs:339:18
    |
    |
339 |     pub const fn get<I>(&self, index: I) -> Option<&I::Output>
help: provide the argument
    |
    |
73  |     if DTORS.get(/* index */).is_null() {

error[E0599]: no method named `is_null` found for enum `core::option::Option` in the current scope
  --> /checkout/library/std/src/sys/unix/thread_local_dtor.rs:73:20
   |
   |
73 |     if DTORS.get().is_null() {
   |                    ^^^^^^^ method not found in `Option<&_>`

error[E0599]: no method named `set` found for struct `alloc_crate::vec::Vec<(*mut u8, unsafe extern "C" fn(*mut u8))>` in the current scope
    |
    |
75  |         DTORS.set(Box::into_raw(v));
    |
   ::: /checkout/library/core/src/pin.rs:745:12
    |
    |
745 |     pub fn set(&mut self, value: P::Target)
    |            |
    |            |
    |            the method is available for `core::pin::Pin<alloc_crate::vec::Vec<(*mut u8, unsafe extern "C" fn(*mut u8))>>` here
    |            the method is available for `core::pin::Pin<&mut alloc_crate::vec::Vec<(*mut u8, unsafe extern "C" fn(*mut u8))>>` here
help: consider wrapping the receiver expression with the appropriate type
    |
    |
75  |         Pin::new(DTORS).set(Box::into_raw(v));
help: consider wrapping the receiver expression with the appropriate type
    |
    |
75  |         Pin::new(&mut DTORS).set(Box::into_raw(v));
help: there is a method with a similar name
    |
    |
75  |         DTORS.get(Box::into_raw(v));

Some errors have detailed explanations: E0061, E0428, E0599.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `std` due to 4 previous errors
