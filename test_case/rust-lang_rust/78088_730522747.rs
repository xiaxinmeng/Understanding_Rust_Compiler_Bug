plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +0912f9f1bc2b701367e8fabc8b09d248520a2f17:refs/remotes/pull/78088/merge
---
    Checking url v2.1.1
    Checking toml v0.5.7
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
error: unused import: `ExprKind`
 --> src/tools/clippy/clippy_lints/src/panic_unimplemented.rs:3:23
  |
3 | use rustc_hir::{Expr, ExprKind};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `clippy_lints`

