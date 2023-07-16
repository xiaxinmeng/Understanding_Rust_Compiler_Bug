plain
2019-10-09T00:18:40.2167719Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T00:18:40.2362209Z ##[command]git config gc.auto 0
2019-10-09T00:18:40.2455182Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T00:18:40.2518259Z ##[command]git config --get-all http.proxy
2019-10-09T00:18:40.2699610Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65219/merge:refs/remotes/pull/65219/merge
---
2019-10-09T00:24:47.9619518Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T00:24:47.9637644Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T00:24:48.3735587Z    Compiling cc v1.0.35
2019-10-09T00:24:48.3735949Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-10-09T00:24:56.0672165Z error[E0271]: type mismatch resolving `<&mut Self as iter::traits::collect::IntoIterator>::Item == B`
2019-10-09T00:24:56.0672664Z     --> src/libcore/iter/traits/iterator.rs:2398:13
2019-10-09T00:24:56.0672902Z      |
2019-10-09T00:24:56.0673203Z 2398 |         rhs.extend(&mut self);
2019-10-09T00:24:56.0673754Z      |
2019-10-09T00:24:56.0674060Z      = note: expected type `(A, B)`
2019-10-09T00:24:56.0674300Z                 found type `B`
2019-10-09T00:24:56.0674591Z      = help: type parameters must be constrained to match other types
2019-10-09T00:24:56.0674591Z      = help: type parameters must be constrained to match other types
2019-10-09T00:24:56.0674957Z      = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
2019-10-09T00:24:56.0728246Z error[E0618]: expected function, found `()`
2019-10-09T00:24:56.0728591Z     --> src/libcore/iter/traits/iterator.rs:2401:9
2019-10-09T00:24:56.0728815Z      |
2019-10-09T00:24:56.0728815Z      |
2019-10-09T00:24:56.0729115Z 2401 |           self.for_each(#[inline(always)] |_| {})
2019-10-09T00:24:56.0729466Z      |  _________-^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-09T00:24:56.0730027Z 2403 | |         (lhs, rhs)
2019-10-09T00:24:56.0730376Z      | |__________________- call expression requires function
2019-10-09T00:24:56.0730420Z 
2019-10-09T00:24:56.1833245Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
---
2019-10-09T00:25:00.9921014Z == clock drift check ==
2019-10-09T00:25:00.9921067Z   local time: Wed Oct  9 00:25:00 UTC 2019
2019-10-09T00:25:01.0763937Z   network time: Wed, 09 Oct 2019 00:25:01 GMT
2019-10-09T00:25:01.0768615Z == end clock drift check ==
2019-10-09T00:25:14.6127189Z ##[error]Bash exited with code '1'.
2019-10-09T00:25:14.6171661Z ##[section]Starting: Checkout
2019-10-09T00:25:14.6173494Z ==============================================================================
2019-10-09T00:25:14.6173566Z Task         : Get sources
2019-10-09T00:25:14.6173613Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
