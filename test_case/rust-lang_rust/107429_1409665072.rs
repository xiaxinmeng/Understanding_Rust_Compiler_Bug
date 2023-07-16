plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:055e3b93d15803815fe6f9cbc1b02b11be094e54)
Complete job name: PR (x86_64-gnu-tools, false, 1, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between a322848c6b0e037c1f0209387558ecb6ab763714 and cf676e4684ef4170339fd04b02fc9e454f637060
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `memchr` is not yet stable as a const fn
   --> library/core/src/ffi/c_str.rs:326:23
    |
326 |         let nul_pos = memchr::memchr(0, bytes);
    |
    = help: const-stable functions can only call other const-stable functions


error: `slice::index::<impl Index<I> for [T]>::index` is not yet stable as a const fn
    |
    |
329 |                 let subslice = &bytes[..nul_pos + 1];
    |
    = help: const-stable functions can only call other const-stable functions

error: could not compile `core` due to 2 previous errors
