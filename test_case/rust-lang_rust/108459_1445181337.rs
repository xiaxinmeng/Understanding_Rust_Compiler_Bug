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
---- [rustdoc] tests/rustdoc/intra-doc/prim-precedence.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-precedence" "/checkout/tests/rustdoc/intra-doc/prim-precedence.rs"
stdout: none
--- stderr -------------------------------
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="char/index.html"]' 'mod@char'
Encountered 1 errors
------------------------------------------


