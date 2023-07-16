plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0284]: type annotations needed
   --> compiler/rustc_data_structures/src/obligation_forest/mod.rs:352:63
    |
352 |                     None => self.obligation_tree_id_generator.next().unwrap(),
    |
    |
    = note: cannot satisfy `<_ as Iterator>::Item == ObligationTreeId`
    |
    |
352 |                     None => <Self as Iterator>::next(self.obligation_tree_id_generator).unwrap(),

For more information about this error, try `rustc --explain E0284`.
error: could not compile `rustc_data_structures` due to previous error
warning: build failed, waiting for other jobs to finish...
