plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +c6f25efdb580494ae8824f78208edb8356e83075:refs/remotes/pull/79173/merge
---
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: variable does not need to be mutable
  --> library/core/src/iter/adapters/zip.rs:30:29
   |
30 |     fn super_nth(&mut self, mut n: usize) -> Option<(A::Item, B::Item)> {
   |                             |
   |                             help: remove this `mut`
   |
   |
   = note: `-D unused-mut` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `core`

