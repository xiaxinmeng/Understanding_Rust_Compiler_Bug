plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:c8a09107e1a7966f8c20565a263305ce8f62405f)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: associated function has missing stability attribute
    --> library/core/src/cell.rs:2000:5
     |
2000 | /     pub const fn from_mut(value: &mut T) -> &mut UnsafeCell<T> {
2001 | |         // SAFETY: `UnsafeCell<T>` has the same memory layout as `T` due to #[repr(transparent)].
2002 | |         unsafe { &mut *(value as *mut T as *mut UnsafeCell<T>) }
     | |_____^

error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
