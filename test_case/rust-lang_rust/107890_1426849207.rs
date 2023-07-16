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
   Compiling rand v0.8.5
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling std v0.0.0 (/checkout/library/std)
error: `Iterator::map` call that discard the iterator's values
     |
     |
1250 |     v.get_mut(1).map(|e| *e = 7);
     |                  |   |
     |                  |   |
     |                  |   this function returns `()`, which is likely not what you wanted
     |                  |   called `Iterator::map` with callable that returns `()`
     |                  after this call to map, the resulting iterator is `impl Iterator<Item = ()>`, which means the only information carried by the iterator is the number of items
     |
     = note: `Iterator::map`, like many of the methods on `Iterator`, gets executed lazily, meaning that its effects won't be visible until it is iterated
     = note: `-D map-unit-fn` implied by `-D warnings`
help: you might have meant to use `Iterator::for_each`
     |
1250 |     v.get_mut(1).for_each( /* code */ );

error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:18:38
