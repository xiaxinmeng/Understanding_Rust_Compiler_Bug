plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
   Compiling getopts v0.2.21
   Compiling rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
   Compiling either v1.6.0
   Compiling convert_case v0.4.0
error[E0283]: type annotations needed for `HashSet<X, S>`
   --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/rand-0.8.5/src/seq/index.rs:528:9
    |
528 |     let mut cache = HashSet::with_capacity(amount.as_usize());
    |         ^^^^^^^^^   ---------------------- type must be known at this point
    = note: cannot satisfy `_: Default`
    = note: cannot satisfy `_: Default`
note: required by a bound in `HashSet::<T, S>::with_capacity`
    |
    |
117 | impl<T, S: Default> HashSet<T, S> {
    |            ^^^^^^^ required by this bound in `HashSet::<T, S>::with_capacity`
help: consider giving `cache` an explicit type, where the type for type parameter `S` is specified
    |
528 |     let mut cache: HashSet<X, S> = HashSet::with_capacity(amount.as_usize());

   Compiling rustc_serialize v0.0.0 (/checkout/compiler/rustc_serialize)
   Compiling md-5 v0.10.0
For more information about this error, try `rustc --explain E0283`.
