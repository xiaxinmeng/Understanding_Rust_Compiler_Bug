plain
2019-11-11T20:45:00.4981659Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T20:45:01.1916861Z ##[command]git config gc.auto 0
2019-11-11T20:45:01.1920627Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T20:45:01.1924190Z ##[command]git config --get-all http.proxy
2019-11-11T20:45:01.1929371Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64588/merge:refs/remotes/pull/64588/merge
---
2019-11-11T21:00:26.6006176Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-11-11T21:01:07.3689500Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-11-11T21:02:08.7300720Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-11-11T21:02:09.9360324Z error[E0531]: cannot find unit struct/variant or constant `MutImmutable` in module `hir`
2019-11-11T21:02:09.9362284Z    --> src/librustc_mir/hair/cx/expr.rs:839:18
2019-11-11T21:02:09.9363621Z 839 |             hir::MutImmutable => Mutability::Not,
2019-11-11T21:02:09.9364180Z     |                  ^^^^^^^^^^^^ not found in `hir`
2019-11-11T21:02:09.9364396Z 
2019-11-11T21:02:09.9408618Z error[E0531]: cannot find unit struct/variant or constant `MutMutable` in module `hir`
2019-11-11T21:02:09.9408618Z error[E0531]: cannot find unit struct/variant or constant `MutMutable` in module `hir`
2019-11-11T21:02:09.9409117Z    --> src/librustc_mir/hair/cx/expr.rs:840:18
2019-11-11T21:02:09.9409927Z 840 |             hir::MutMutable => Mutability::Mut,
2019-11-11T21:02:09.9410355Z     |                  ^^^^^^^^^^ not found in `hir`
2019-11-11T21:02:09.9410752Z 
2019-11-11T21:02:18.5696664Z error: aborting due to 2 previous errors
---
2019-11-11T21:03:25.5264550Z   local time: Mon Nov 11 21:03:25 UTC 2019
2019-11-11T21:03:25.6776340Z   network time: Mon, 11 Nov 2019 21:03:25 GMT
2019-11-11T21:03:25.6779054Z == end clock drift check ==
2019-11-11T21:03:28.4244815Z 
2019-11-11T21:03:28.4334821Z ##[error]Bash exited with code '1'.
2019-11-11T21:03:28.4361377Z ##[section]Starting: Checkout
2019-11-11T21:03:28.4363812Z ==============================================================================
2019-11-11T21:03:28.4363890Z Task         : Get sources
2019-11-11T21:03:28.4363940Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
