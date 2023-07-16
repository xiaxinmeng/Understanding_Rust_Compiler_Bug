plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +dc793e66ca1831a040a7b514b362f584ff02854d:refs/remotes/pull/79000/merge
---
    Checking clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/attrs.rs:430:29
    |
430 | ...                   symbols.iter(),
    |                       ^^^^^^^^^^^^^^ expected `&[Symbol]`, found struct `std::slice::Iter`
    |
    = note: expected reference `&[Symbol]`
                  found struct `std::slice::Iter<'_, Symbol>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints`
---
== clock drift check ==
  local time: Thu Nov 12 20:11:45 UTC 2020
  network time: Thu, 12 Nov 2020 09:14:37 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
[command]/usr/bin/git version
git version 2.29.2
[command]/usr/bin/git config --local --name-only --get-regexp core\.sshCommand
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :
