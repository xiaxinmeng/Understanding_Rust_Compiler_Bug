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
  --> /checkout/library/std/src/sys/unix/thread_local_dtor.rs:71:5
   |
61 |     static mut DTORS: PlVec<(*mut u8, unsafe extern "C" fn(*mut u8))> = PlVec::new();
   |     --------------------------------------------------------------------------------- previous definition of the value `DTORS` here
...
71 |     static DTORS: Cell<*mut List> = Cell::new(ptr::null_mut());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `DTORS` redefined here
   |
   = note: `DTORS` must be defined only once in the value namespace of this block
error[E0412]: cannot find type `PlVec` in this scope
  --> /checkout/library/std/src/sys/unix/thread_local_dtor.rs:61:23
   |
   |
61 |     static mut DTORS: PlVec<(*mut u8, unsafe extern "C" fn(*mut u8))> = PlVec::new();
   |
help: consider importing one of these items
   |
14 | use alloc::vec::PlVec;
14 | use alloc::vec::PlVec;
   |
14 | use crate::vec::PlVec;
   |

error[E0433]: failed to resolve: use of undeclared type `PlVec`
  --> /checkout/library/std/src/sys/unix/thread_local_dtor.rs:61:73
   |
61 |     static mut DTORS: PlVec<(*mut u8, unsafe extern "C" fn(*mut u8))> = PlVec::new();
   |                                                                         ^^^^^ use of undeclared type `PlVec`
help: consider importing one of these items
   |
14 | use alloc::vec::PlVec;
   |
