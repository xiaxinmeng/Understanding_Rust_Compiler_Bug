plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +cb6b9b1475af4d1792e704b7b1790ff52a8c13fe:refs/remotes/pull/80090/merge
---
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/lib.rs:553:25
    |
553 |                         resolver,
    |                         ^^^^^^^^ expected struct `Rc`, found struct `Arc`
    |
    = note: expected struct `Rc<RefCell<BoxedResolver>>`
               found struct `Arc<RefCell<BoxedResolver>>`
error[E0308]: mismatched types
   --> src/librustdoc/core.rs:440:5
    |
    |
407 | crate fn create_resolver<'a>(externs: config::Externs, queries: &Queries<'a>, sess: &Session) -> Lrc<RefCell<interface::BoxedResolver>> {
    |                                                                                                  -------------------------------------- expected `Arc<RefCell<BoxedResolver>>` because of return type
440 |     resolver.clone()
440 |     resolver.clone()
    |     ^^^^^^^^^^^^^^^^ expected struct `Arc`, found struct `Rc`
    |
    = note: expected struct `Arc<RefCell<BoxedResolver>>`
               found struct `Rc<RefCell<BoxedResolver>>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc`
