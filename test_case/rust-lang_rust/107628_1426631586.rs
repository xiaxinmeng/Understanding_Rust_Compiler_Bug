plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:c8a09107e1a7966f8c20565a263305ce8f62405f)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
DirectMap1G:    54525952 kB
Building bootstrap
    Finished dev [unoptimized] target(s) in 0.06s
Building stage0 library artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: could not execute process `rustc -vV` (never executed)
Caused by:
  No such file or directory (os error 2)
Build completed unsuccessfully in 0:00:00
cat: /tmp/toolstate/toolstates.json: No such file or directory
