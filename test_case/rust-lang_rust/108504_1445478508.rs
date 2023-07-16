plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0170]: pattern binding `dep_kind` is named the same as one of the variants of the type `dep_graph::dep_node::DepKind`
  --> compiler/rustc_middle/src/dep_graph/mod.rs:97:29
   |
97 |     fn dep_kind_info(&self, dep_kind: DepKind) -> &DepKindStruct<'tcx> {
   |
   |
   = note: `#[deny(bindings_with_variant_name)]` on by default
For more information about this error, try `rustc --explain E0170`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:06:12
