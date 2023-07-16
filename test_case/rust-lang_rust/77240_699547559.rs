plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/77240/merge:refs/remotes/pull/77240/merge
---
   Compiling libc v0.2.77
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.35
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0599]: no function or associated item named `first_ptr_mut` found for union `mem::maybe_uninit::MaybeUninit<_>` in the current scope
   --> library/core/src/fmt/num.rs:484:32
    |
484 |     let buf_ptr = MaybeUninit::first_ptr_mut(buf);
    |                                ^^^^^^^^^^^^^ function or associated item not found in `mem::maybe_uninit::MaybeUninit<_>`
   ::: library/core/src/mem/maybe_uninit.rs:221:1
    |
221 | pub union MaybeUninit<T> {
221 | pub union MaybeUninit<T> {
    | ------------------------ function or associated item `first_ptr_mut` not found for this

error[E0599]: no function or associated item named `first_ptr_mut` found for union `mem::maybe_uninit::MaybeUninit<_>` in the current scope
   --> library/core/src/fmt/num.rs:598:32
    |
598 |     let buf_ptr = MaybeUninit::first_ptr_mut(&mut buf);
    |                                ^^^^^^^^^^^^^ function or associated item not found in `mem::maybe_uninit::MaybeUninit<_>`
   ::: library/core/src/mem/maybe_uninit.rs:221:1
    |
221 | pub union MaybeUninit<T> {
221 | pub union MaybeUninit<T> {
    | ------------------------ function or associated item `first_ptr_mut` not found for this
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `core`.
---
== clock drift check ==
  local time: Sat Sep 26 21:03:05 UTC 2020
  network time: Sat, 26 Sep 2020 21:03:05 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (2947) (node)
Terminate orphan process: pid (2956) (python)
