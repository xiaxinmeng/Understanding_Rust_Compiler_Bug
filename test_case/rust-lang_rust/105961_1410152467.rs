plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:055e3b93d15803815fe6f9cbc1b02b11be094e54)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.85
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0391]: cycle detected when computing predicates of `array::iter::<impl at library/core/src/array/iter.rs:241:1: 241:52>::fold`
    |
    |
266 | /     fn fold<Acc, Fold>(mut self, init: Acc, mut fold: Fold) -> Acc
267 | |     where
268 | |         Fold: FnMut(Acc, Self::Item) -> Acc,
    |
    |
note: ...which requires computing predicates of `array::iter::<impl at library/core/src/array/iter.rs:241:1: 241:52>::fold`...
    |
    |
266 | /     fn fold<Acc, Fold>(mut self, init: Acc, mut fold: Fold) -> Acc
267 | |     where
268 | |         Fold: FnMut(Acc, Self::Item) -> Acc,
    | |____________________________________________^
note: ...which requires computing explicit predicates of `array::iter::<impl at library/core/src/array/iter.rs:241:1: 241:52>::fold`...
    |
    |
266 | /     fn fold<Acc, Fold>(mut self, init: Acc, mut fold: Fold) -> Acc
267 | |     where
268 | |         Fold: FnMut(Acc, Self::Item) -> Acc,
    | |____________________________________________^
note: ...which requires computing normalized predicates of `array::iter::<impl at library/core/src/array/iter.rs:241:1: 241:52>::fold`...
    |
    |
266 | /     fn fold<Acc, Fold>(mut self, init: Acc, mut fold: Fold) -> Acc
267 | |     where
268 | |         Fold: FnMut(Acc, Self::Item) -> Acc,
    | |____________________________________________^
    = note: ...which again requires computing predicates of `array::iter::<impl at library/core/src/array/iter.rs:241:1: 241:52>::fold`, completing the cycle
    = note: cycle used when checking effective visibilities
For more information about this error, try `rustc --explain E0391`.
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:30
