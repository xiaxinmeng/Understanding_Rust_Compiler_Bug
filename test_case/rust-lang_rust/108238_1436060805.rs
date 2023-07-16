plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
   Compiling rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error: reached the recursion limit while instantiating `<std::iter::Chain<std::slice::Iter<'_, hir::Pat<'_>>, std::option::IntoIter<&...>> as Iterator>::fold::<..., ...>`
   |
   |
89 |             acc = a.fold(acc, &mut f);
   |
   |
note: `<std::iter::Chain<A, B> as Iterator>::fold` defined here
   |
   |
84 | /     fn fold<Acc, F>(self, mut acc: Acc, mut f: F) -> Acc
85 | |     where
86 | |         F: FnMut(Acc, Self::Item) -> Acc,
   | |_________________________________________^
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_hir-2a77d4b64ffcc8b1.long-type.txt'
   Compiling rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error: could not compile `rustc_hir` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:20
