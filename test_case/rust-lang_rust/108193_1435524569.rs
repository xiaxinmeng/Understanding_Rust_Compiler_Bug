plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
---- [rustdoc-json] tests/rustdoc-json/primitives/use_primitive.rs stdout ----

error: jsondoclint failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondoclint" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/primitives/use_primitive/use_primitive.json"
stdout: none
--- stderr -------------------------------
2:28776:33502 not in index or paths, but refered to at '$.index["2:29402:34671"].links["isize::rem_ceil"]'
Error: Errors validating json /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/primitives/use_primitive/use_primitive.json



failures:
