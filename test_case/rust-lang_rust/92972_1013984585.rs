plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between bd3cb52565faab2755ff1bdb54d88bc91f47b4b9 and 42ab9d928e401ab452da70cf2ade6e05ddb8eaa3
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling rustfmt-nightly v1.4.38 (/checkout/src/tools/rustfmt)
error: field is never read: `1`
    --> src/tools/rustfmt/src/expr.rs:1857:31
     |
1857 |     Expr(&'ast ast::ExprKind, Span),
     |
     |
     = note: `-D dead-code` implied by `-D warnings`
error: could not compile `rustfmt-nightly` due to previous error
error: could not compile `rustfmt-nightly` due to previous error
thread 'main' panicked at 'in-tree tool', src/bootstrap/test.rs:384:14
Build completed unsuccessfully in 0:00:09
