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
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: binary operation on reference to `Copy` type `rustc_span::Symbol`
    --> src/librustdoc/clean/mod.rs:1391:78
     |
1391 |                     }) = generics.params.iter_mut().find(|param| &param.name == arg)
     |
     |
     = note: `rustc_span::Symbol` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
     = note: `#[deny(ref_binop_on_copy_type)]` on by default
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
     |
1391 -                     }) = generics.params.iter_mut().find(|param| &param.name == arg)
1391 +                     }) = generics.params.iter_mut().find(|param| param.name == *arg)

error: binary operation on reference to `Copy` type `rustc_span::Symbol`
    --> src/librustdoc/clean/mod.rs:1398:78
     |
     |
1398 |                     }) = generics.params.iter_mut().find(|param| &param.name == arg) {
     |
     |
     = note: `rustc_span::Symbol` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
     |
1398 -                     }) = generics.params.iter_mut().find(|param| &param.name == arg) {
1398 +                     }) = generics.params.iter_mut().find(|param| param.name == *arg) {

error: binary operation on reference to `Copy` type `rustc_ast::Mutability`
    --> src/librustdoc/clean/types.rs:1486:28
     |
     |
1486 |                 mutability == b_mutability && type_.is_same(b_type_, cache)
     |
     |
     = note: `rustc_ast::Mutability` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
     |
1486 |                 *mutability == *b_mutability && type_.is_same(b_type_, cache)

error: binary operation on reference to `Copy` type `rustc_ast::Mutability`
    --> src/librustdoc/clean/types.rs:1491:29
     |
     |
1491 |             ) => mutability == b_mutability && type_.is_same(b_type_, cache),
     |
     |
     = note: `rustc_ast::Mutability` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
     |
1491 |             ) => *mutability == *b_mutability && type_.is_same(b_type_, cache),

error: binary operation on reference to `Copy` type `rustc_span::Symbol`
   --> src/librustdoc/html/format.rs:747:14
    |
    |
747 |         if f != r {
    |
    |
    = note: `rustc_span::Symbol` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
747 |         if *f != *r {

error: binary operation on reference to `Copy` type `rustc_span::Symbol`
   --> src/librustdoc/html/render/context.rs:231:77
    |
    |
231 |                     || self.current.iter().zip(names.iter()).any(|(a, b)| a != b)
    |
    |
    = note: `rustc_span::Symbol` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
231 |                     || self.current.iter().zip(names.iter()).any(|(a, b)| *a != *b)

error: could not compile `rustdoc` due to 6 previous errors
Build completed unsuccessfully in 0:09:04
cat: /tmp/toolstate/toolstates.json: No such file or directory
