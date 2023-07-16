plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
   Compiling core v0.0.0 (/checkout/library/core)
error: missing documentation for a function
   --> library/alloc/src/slice.rs:100:5
    |
100 |     pub fn into_vec<T, A: Allocator>(b: Box<[T], A>) -> Vec<T, A> {
    |
note: because it is re-exported here
   --> library/alloc/src/slice.rs:80:1
    |
    |
80  | pub use hack::into_vec;
    | ^^^^^^^^^^^^^^^^^^^^^^^
    = note: `-D missing-docs` implied by `-D warnings`
error: missing documentation for a function
   --> library/alloc/src/slice.rs:110:5
    |
    |
110 |     pub fn to_vec<T: ConvertVec, A: Allocator>(s: &[T], alloc: A) -> Vec<T, A> {
    |
note: because it is re-exported here
   --> library/alloc/src/slice.rs:85:1
    |
