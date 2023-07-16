plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +d72038a0b80906e8951ec23265d0e3c0860dbcf9:refs/remotes/pull/79877/merge
---
    Checking rustc-rayon v0.3.0
    Checking tempfile v3.1.0
    Checking regex v1.3.9
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0277]: the trait bound `str: __Deref` is not satisfied
    --> src/librustdoc/html/render/mod.rs:2282:26
     |
2282 |                 if since == "TBD" {
     |                          ^^ the trait `__Deref` is not implemented for `str`
     |
     = note: required because of the requirements on the impl of `std::cmp::PartialEq<str>` for `SymbolStr`
     = note: required because of the requirements on the impl of `std::cmp::PartialEq<&str>` for `&SymbolStr`

error[E0277]: the size for values of type `str` cannot be known at compilation time
    --> src/librustdoc/html/render/mod.rs:2282:26
     |
2282 |                 if since == "TBD" {
     |                          ^^ doesn't have a size known at compile-time
     |
     = help: the trait `std::marker::Sized` is not implemented for `str`
     = note: required because of the requirements on the impl of `std::cmp::PartialEq<str>` for `SymbolStr`
     = note: required because of the requirements on the impl of `std::cmp::PartialEq<&str>` for `&SymbolStr`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustdoc`
