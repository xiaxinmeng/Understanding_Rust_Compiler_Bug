plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:c8a09107e1a7966f8c20565a263305ce8f62405f)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
....................i................................................................... 3960/3988
............................
failures:

---- src/iter/mod.rs - iter (line 279) stdout ----
error: `Iterator::map` call that discard the iterator's values
  |
  |
6 | v.iter().map(|x| println!("{x}"));
  |          |   |
  |          |   |
  |          |   this function returns `()`, which is likely not what you wanted
  |          |   called `Iterator::map` with callable that returns `()`
  |          after this call to map, the resulting iterator is `impl Iterator<Item = ()>`, which means the only information carried by the iterator is the number of items
  |
  = note: `Iterator::map`, like many of the methods on `Iterator`, gets executed lazily, meaning that its effects won't be visible until it is iterated
 --> src/iter/mod.rs:277:9
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  |         ^^^^^^^^
  = note: `#[deny(map_unit_fn)]` implied by `#[deny(warnings)]`
help: you might have meant to use `Iterator::for_each`
  |
6 | v.iter().for_each( /* code */ );

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.

failures:
    src/iter/mod.rs - iter (line 279)

test result: FAILED. 3950 passed; 1 failed; 37 ignored; 0 measured; 0 filtered out; finished in 59.13s

error: doctest failed, to rerun pass `-p core --doc`
