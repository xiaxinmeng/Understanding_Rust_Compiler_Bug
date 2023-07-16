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
   |
87 |     pub us: String,
   |             ^^^^^^
   |
   = note: this could indicate incorrectly eagerly converting to a string
   = note: `-D rustc::string-in-diagnostic` implied by `-D warnings`
error: use of String in diagnostic
  --> compiler/rustc_session/src/errors.rs:93:13
   |
93 |     pub us: String,
---

error: use of String in diagnostic
   --> compiler/rustc_session/src/errors.rs:255:17
    |
255 |     pub suffix: String,
    |
    = note: this could indicate incorrectly eagerly converting to a string

error: use of String in diagnostic
---

error: use of String in diagnostic
   --> compiler/rustc_session/src/errors.rs:274:17
    |
274 |     pub suffix: String,
    |
    = note: this could indicate incorrectly eagerly converting to a string

error: use of String in diagnostic
