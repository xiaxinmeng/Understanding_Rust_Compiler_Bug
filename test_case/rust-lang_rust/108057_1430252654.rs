plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7ea263296c702710d7fac3c6d5bccdb2895b4e79)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   |
11 | pub use Foo2 as Bar;
   |         ^^^^^^^^^^^ no external crate `Foo2`

error[E0658]: `#[doc(cfg)]` is experimental
 --> clean/mod.rs:2151:1
  |
3 | #[doc(hidden, cfg(feature = "foo"))]
  |
  = note: see issue #43781 <https://github.com/rust-lang/rust/issues/43781> for more information
  = note: see issue #43781 <https://github.com/rust-lang/rust/issues/43781> for more information
  = help: add `#![feature(doc_cfg)]` to the crate attributes to enable

error[E0658]: `#[doc(cfg)]` is experimental
 --> clean/mod.rs:2154:1
  |
6 | #[doc(cfg(feature = "bar"))]
  |
  = note: see issue #43781 <https://github.com/rust-lang/rust/issues/43781> for more information
  = note: see issue #43781 <https://github.com/rust-lang/rust/issues/43781> for more information
  = help: add `#![feature(doc_cfg)]` to the crate attributes to enable
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0432, E0658.
For more information about an error, try `rustc --explain E0432`.
For more information about an error, try `rustc --explain E0432`.
Couldn't compile the test.

failures:
    clean/mod.rs - clean::add_without_unwanted_attributes (line 2150)

test result: FAILED. 3 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out; finished in 1.65s

error: doctest failed, to rerun pass `-p rustdoc --doc`
