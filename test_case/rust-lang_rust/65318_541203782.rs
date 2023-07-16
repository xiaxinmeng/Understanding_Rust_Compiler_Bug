plain
2019-10-11T19:39:45.8960196Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-11T19:39:45.9242665Z ##[command]git config gc.auto 0
2019-10-11T19:39:45.9312261Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-11T19:39:45.9392255Z ##[command]git config --get-all http.proxy
2019-10-11T19:39:45.9543278Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65318/merge:refs/remotes/pull/65318/merge
---
2019-10-11T19:48:45.3668195Z error: unused variable: `types`
2019-10-11T19:48:45.3669307Z   --> src/librustc_typeck/coherence/orphan.rs:27:52
2019-10-11T19:48:45.3669581Z    |
2019-10-11T19:48:45.3669912Z 27 |         if let hir::ItemKind::Impl(.., Some(_), _, types) = item.kind {
2019-10-11T19:48:45.3670265Z    |                                                    ^^^^^ help: consider prefixing with an underscore: `_types`
2019-10-11T19:48:45.3670913Z    = note: `-D unused-variables` implied by `-D warnings`
2019-10-11T19:48:45.3670953Z 
2019-10-11T19:48:45.3670953Z 
2019-10-11T19:48:48.1109281Z error[E0507]: cannot move out of `item.kind.6` which is behind a shared reference
2019-10-11T19:48:48.1109702Z   --> src/librustc_typeck/coherence/orphan.rs:27:61
2019-10-11T19:48:48.1110249Z 27 |         if let hir::ItemKind::Impl(.., Some(_), _, types) = item.kind {
2019-10-11T19:48:48.1111334Z    |                                                    -----    ^^^^^^^^^ help: consider borrowing here: `&item.kind`
2019-10-11T19:48:48.1111803Z    |                                                    |
2019-10-11T19:48:48.1112077Z    |                                                    data moved here
2019-10-11T19:48:48.1112077Z    |                                                    data moved here
2019-10-11T19:48:48.1112465Z    |                                                    move occurs because `types` has type `rustc::hir::ptr::P<[rustc::hir::ImplItemRef]>`, which does not implement the `Copy` trait
2019-10-11T19:48:48.4902798Z error: aborting due to 2 previous errors
2019-10-11T19:48:48.4902991Z 
2019-10-11T19:48:48.4908796Z For more information about this error, try `rustc --explain E0507`.
2019-10-11T19:48:48.5513342Z error: could not compile `rustc_typeck`.
---
2019-10-11T19:49:02.2584121Z == clock drift check ==
2019-10-11T19:49:02.2602526Z   local time: Fri Oct 11 19:49:02 UTC 2019
2019-10-11T19:49:02.3346021Z   network time: Fri, 11 Oct 2019 19:49:02 GMT
2019-10-11T19:49:02.3349275Z == end clock drift check ==
2019-10-11T19:49:03.1962067Z ##[error]Bash exited with code '1'.
2019-10-11T19:49:03.2005875Z ##[section]Starting: Checkout
2019-10-11T19:49:03.2007825Z ==============================================================================
2019-10-11T19:49:03.2007883Z Task         : Get sources
2019-10-11T19:49:03.2007930Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
