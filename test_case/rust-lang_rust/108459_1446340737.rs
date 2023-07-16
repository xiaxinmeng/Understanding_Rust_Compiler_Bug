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
---- [rustdoc] tests/rustdoc/intra-doc/issue-108459.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/issue-108459" "/checkout/tests/rustdoc/intra-doc/issue-108459.rs"
stdout: none
--- stderr -------------------------------
15: @has check failed
 File does not exist 'issue-108459/struct.MyStruct2.html'
 // @has issue-108459/struct.MyStruct2.html '//*[@href="char/index.html"]' 'crate::char'
16: @has check failed
 File does not exist 'issue-108459/struct.MyStruct2.html'
 // @has - '//*[@href="char/index.html"]' 'char'
17: @has check failed
 File does not exist 'issue-108459/struct.MyStruct2.html'
 // @has - '//*[@href="{{channel}}/std/primitive.char.html"]' 'char'
21: @has check failed
 File does not exist 'issue-108459/struct.MyStruct3.html'
 // @has issue-108459/struct.MyStruct3.html '//*[@href="char/index.html"]' 'crate::char'
22: @has check failed
 File does not exist 'issue-108459/struct.MyStruct3.html'
 // @has - '//*[@href="char/index.html"]' 'char'
23: @has check failed
 File does not exist 'issue-108459/struct.MyStruct3.html'
 // @has - '//*[@href="{{channel}}/std/primitive.char.html"]' 'char'
30: @has check failed
 File does not exist 'issue-108459/struct.MyStruct4.html'
 // @has issue-108459/struct.MyStruct4.html '//*[@href="char/index.html"]' 'char'
31: @has check failed
 File does not exist 'issue-108459/struct.MyStruct4.html'
 // @has - '//*[@href="{{channel}}/std/primitive.char.html"]' 'char'
35: @has check failed
 File does not exist 'issue-108459/struct.MyStruct5.html'
 // @has issue-108459/struct.MyStruct5.html '//*[@href="char/index.html"]' 'char'
36: @has check failed
 File does not exist 'issue-108459/struct.MyStruct5.html'
 // @has - '//*[@href="{{channel}}/std/primitive.char.html"]' 'char'
Encountered 10 errors
------------------------------------------


