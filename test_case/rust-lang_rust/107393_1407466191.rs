plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   Compiling askama_shared v0.12.0
   Compiling askama_derive v0.11.0
   Compiling askama v0.11.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: using the a comparison operator on `Ty`
   --> src/librustdoc/clean/simplify.rs:136:45
    |
136 |                 if pred.trait_ref.self_ty() == self_ty { Some(pred.def_id()) } else { None }
    |
    |
    = note: this does probably not what you want as it does not handle inference variables and more
    = note: it's also not recommended to use it for diagnostics
    = help: for more information, see https://rustc-dev-guide.rust-lang.org/ty.html#comparing-types
    = note: `-D rustc::ty-compare-operator` implied by `-D warnings`

error: using the a comparison operator on `Ty`
    --> src/librustdoc/clean/mod.rs:1261:32
     |
1261 |                 if self_arg_ty == self_ty {
     |
     |
     = note: this does probably not what you want as it does not handle inference variables and more
     = note: it's also not recommended to use it for diagnostics
     = help: for more information, see https://rustc-dev-guide.rust-lang.org/ty.html#comparing-types

error: using the a comparison operator on `Ty`
    --> src/librustdoc/clean/mod.rs:1264:27
1264 |                     if ty == self_ty {
     |                           ^^
     |
     |
     = note: this does probably not what you want as it does not handle inference variables and more
     = note: it's also not recommended to use it for diagnostics
     = help: for more information, see https://rustc-dev-guide.rust-lang.org/ty.html#comparing-types

error: using the a comparison operator on `Ty`
    |
    |
802 |             let saw_impl = impl_type == ty
    |
    |
    = note: this does probably not what you want as it does not handle inference variables and more
    = note: it's also not recommended to use it for diagnostics
    = help: for more information, see https://rustc-dev-guide.rust-lang.org/ty.html#comparing-types
error: could not compile `rustdoc` due to 4 previous errors
Build completed unsuccessfully in 0:15:59
