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
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: binary operation on reference to `Copy` type `rustc_middle::ty::AdtDef<'_>`
    --> compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1968:65
     |
1968 |                 (ty::Adt(def_a, _), ty::Adt(def_b, _)) => def_a == def_b,
     |
     |
     = note: `rustc_middle::ty::AdtDef<'_>` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
     = note: `#[deny(ref_binop_on_copy_type)]` on by default
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
     |
1968 |                 (ty::Adt(def_a, _), ty::Adt(def_b, _)) => *def_a == *def_b,

error: binary operation on reference to `Copy` type `rustc_span::def_id::DefId`
    --> compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1969:67
     |
     |
1969 |                 (ty::Foreign(def_a), ty::Foreign(def_b)) => def_a == def_b,
     |
     |
     = note: `rustc_span::def_id::DefId` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
     |
1969 |                 (ty::Foreign(def_a), ty::Foreign(def_b)) => *def_a == *def_b,

error: binary operation on reference to `Copy` type `rustc_middle::ty::Ty<'_>`
    --> compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:2852:51
     |
     |
2852 |             if obligated_types.iter().any(|ot| ot == &self_ty) {
     |
     |
     = note: `rustc_middle::ty::Ty<'_>` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
     |
2852 -             if obligated_types.iter().any(|ot| ot == &self_ty) {
2852 +             if obligated_types.iter().any(|ot| *ot == self_ty) {

error: binary operation on reference to `Copy` type `rustc_middle::ty::AdtDef<'_>`
    --> compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:2859:30
     |
     |
2859 |                 && inner_def == def
     |
     |
     = note: `rustc_middle::ty::AdtDef<'_>` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
     |
2859 |                 && *inner_def == *def

error: binary operation on reference to `Copy` type `usize`
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2106:42
     |
     |
2106 | ...                   && other_idx != idx
     |
     |
     = note: `usize` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
     |
2106 |                             && *other_idx != *idx

error: binary operation on reference to `Copy` type `usize`
    --> compiler/rustc_trait_selection/src/traits/select/mod.rs:1970:19
     |
     |
1970 |                 i < j && !needs_infer
     |
     |
     = note: `usize` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
     |
1970 |                 *i < *j && !needs_infer

error: binary operation on reference to `Copy` type `rustc_middle::ty::Ty<'_>`
   --> compiler/rustc_trait_selection/src/traits/wf.rs:954:26
    |
    |
954 |                     if t == &erased_self_ty && !r.has_escaping_bound_vars() {
    |
    |
    = note: `rustc_middle::ty::Ty<'_>` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
954 -                     if t == &erased_self_ty && !r.has_escaping_bound_vars() {
954 +                     if *t == erased_self_ty && !r.has_escaping_bound_vars() {

error: could not compile `rustc_trait_selection` due to 7 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:08:29
