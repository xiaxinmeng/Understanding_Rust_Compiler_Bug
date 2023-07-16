plain
2019-12-12T22:54:07.0534274Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-12T22:54:07.9223143Z ##[command]git config gc.auto 0
2019-12-12T22:54:07.9228938Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-12T22:54:07.9232972Z ##[command]git config --get-all http.proxy
2019-12-12T22:54:07.9237206Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67116/merge:refs/remotes/pull/67116/merge
---
2019-12-12T23:01:32.4423668Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-12-12T23:02:01.5327205Z error: variable does not need to be mutable
2019-12-12T23:02:01.5327787Z     --> src/librustc/traits/error_reporting.rs:2191:14
2019-12-12T23:02:01.5328018Z      |
2019-12-12T23:02:01.5328514Z 2191 |         let (mut trait_ref, mut target_ty) = (trait_predicate.trait_ref, trait_predicate.self_ty());
2019-12-12T23:02:01.5329096Z      |              |
2019-12-12T23:02:01.5329584Z      |              help: remove this `mut`
2019-12-12T23:02:01.5330094Z      |
2019-12-12T23:02:01.5330358Z      = note: `-D unused-mut` implied by `-D warnings`
2019-12-12T23:02:01.5330358Z      = note: `-D unused-mut` implied by `-D warnings`
2019-12-12T23:02:01.5330396Z 
2019-12-12T23:02:01.5330647Z error: variable does not need to be mutable
2019-12-12T23:02:01.5330917Z     --> src/librustc/traits/error_reporting.rs:2191:29
2019-12-12T23:02:01.5331120Z      |
2019-12-12T23:02:01.5331541Z 2191 |         let (mut trait_ref, mut target_ty) = (trait_predicate.trait_ref, trait_predicate.self_ty());
2019-12-12T23:02:01.5332469Z      |                             |
2019-12-12T23:02:01.5332780Z      |                             help: remove this `mut`
2019-12-12T23:02:01.5332820Z 
2019-12-12T23:02:01.5339166Z error: variable does not need to be mutable
---
2019-12-12T23:02:10.3247711Z   local time: Thu Dec 12 23:02:10 UTC 2019
2019-12-12T23:02:10.6042743Z   network time: Thu, 12 Dec 2019 23:02:10 GMT
2019-12-12T23:02:10.6046516Z == end clock drift check ==
2019-12-12T23:02:11.2381188Z 
2019-12-12T23:02:11.2499050Z ##[error]Bash exited with code '1'.
2019-12-12T23:02:11.2528338Z ##[section]Starting: Checkout
2019-12-12T23:02:11.2529930Z ==============================================================================
2019-12-12T23:02:11.2529982Z Task         : Get sources
2019-12-12T23:02:11.2530043Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
