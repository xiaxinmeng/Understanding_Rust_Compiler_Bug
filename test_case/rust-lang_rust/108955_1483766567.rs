plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:917222de331afc95ef8d3a6300048017039b2b08)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
  |
1 | #![feature(staged_api)]
  |            ^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: could not document `staged_api`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc --edition=2021 --crate-type lib --crate-name staged_api lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc --cfg 'feature="default"' --cfg 'feature="some_feature"' --cfg 'feature="some_other_feature"' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=3f956dfab1ba9863 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/debug/deps --crate-version 0.1.0` (exit status: 1)
