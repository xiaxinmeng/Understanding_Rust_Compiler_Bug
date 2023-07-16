plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/79075/merge:refs/remotes/pull/79075/merge
---
   Compiling hashbrown v0.9.0
   Compiling miniz_oxide v0.4.0
   Compiling object v0.20.0
   Compiling addr2line v0.13.0
error[E0599]: no function or associated item named `static_ref` found for struct `core::pin::Pin<_>` in the current scope
    |
    |
568 |         inner: Pin::static_ref(&INSTANCE).get_or_init_pin(
    |                     ^^^^^^^^^^ function or associated item not found in `core::pin::Pin<_>`

error[E0599]: no function or associated item named `static_ref` found for struct `core::pin::Pin<_>` in the current scope
    |
    |
561 |             if let Some(lock) = Pin::static_ref(instance).try_lock() {
    |                                      ^^^^^^^^^^ function or associated item not found in `core::pin::Pin<_>`

error[E0599]: no function or associated item named `static_ref` found for struct `core::pin::Pin<_>` in the current scope
    |
    |
768 |         inner: Pin::static_ref(&INSTANCE).get_or_init_pin(
    |                     ^^^^^^^^^^ function or associated item not found in `core::pin::Pin<_>`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `std`.
