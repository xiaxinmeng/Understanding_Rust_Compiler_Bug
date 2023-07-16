plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +f1988ba317502747dc704f9690586b3753fd4ca1:refs/remotes/pull/78088/merge
---
    Checking url v2.1.1
    Checking toml v0.5.7
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
error: unused variable: `params`
  --> src/tools/clippy/clippy_lints/src/panic_unimplemented.rs:76:21
   |
76 |         if let Some(params) = match_panic_call(cx, expr) {
   |                     ^^^^^^ help: if this is intentional, prefix it with an underscore: `_params`
   |
   = note: `-D unused-variables` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `clippy_lints`

