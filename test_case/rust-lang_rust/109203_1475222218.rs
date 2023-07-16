plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error[E0308]: mismatched types
   --> compiler/rustc_parse/src/parser/pat.rs:396:54
    |
396 |             || (self.is_lit_bad_ident().is_some() && self)
    |                 ---------------------------------    ^^^^ expected `bool`, found `&mut Parser<'_>`
    |                 expected because this is `bool`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_parse` due to previous error
