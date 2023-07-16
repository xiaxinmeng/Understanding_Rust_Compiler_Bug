plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7b992ce41a6506cf6388fd9a3d24a6eaebaa1d31)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
---- [rustdoc] tests/rustdoc/glob-shadowing.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/glob-shadowing" "/checkout/tests/rustdoc/glob-shadowing.rs"
stdout: none
--- stderr -------------------------------
2: @count check failed
 Expected 6 occurrences but found 7
 // @count - '//div[@class="item-left"]' 6
3: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//div[@class="item-right docblock-short"]' 'sub1::describe'
6: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//div[@class="item-right docblock-short"]' 'sub1::describe2'
Encountered 3 errors
------------------------------------------


