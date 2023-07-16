plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
   Compiling askama_derive v0.12.1
   Compiling askama v0.12.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: using `clone` on a double-reference, which copies the reference of type `clean::types::GenericBound` instead of cloning the inner type
   --> src/librustdoc/html/format.rs:174:70
    |
174 |         for (i, bound) in bounds.iter().filter(|b| bounds_dup.insert(b.clone())).enumerate() {
    |
    |
    = note: `-D clone-double-ref` implied by `-D warnings`
help: try dereferencing it
    |
174 |         for (i, bound) in bounds.iter().filter(|b| bounds_dup.insert(&(*b).clone())).enumerate() {
    |                                                                      +++ ~~~~~~~~~
help: or try being explicit if you are sure, that you want to clone a reference
    |
174 |         for (i, bound) in bounds.iter().filter(|b| bounds_dup.insert(<&clean::types::GenericBound>::clone(b))).enumerate() {

error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:08:56
cat: /tmp/toolstate/toolstates.json: No such file or directory
