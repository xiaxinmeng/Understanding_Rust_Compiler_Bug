
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-fortanix-unknown-sgx)
   Compiling std v0.0.0 (/home/jenkins/workspace/rust-sgx-ci/rust/src/libstd)
error: use of deprecated item 'core::mem::MaybeUninit::<T>::set': use `write` instead
   --> src/libstd/sys/sgx/rwlock.rs:271:18
    |
271 |             init.set(RWLock::new());
    |                  ^^^
    |
= note: `-D deprecated` implied by `-D warnings`
