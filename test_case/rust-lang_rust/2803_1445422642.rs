plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
    Finished dev [unoptimized] target(s) in 0.04s
Skipping Set({test::src/tools/tidy}) because it is excluded
Skipping Set({test::src/tools/tidy}) because it is excluded
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
