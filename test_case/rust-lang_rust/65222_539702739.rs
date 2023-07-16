plain
2019-10-08T20:54:11.9250577Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-08T20:54:11.9449566Z ##[command]git config gc.auto 0
2019-10-08T20:54:11.9516487Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-08T20:54:11.9580236Z ##[command]git config --get-all http.proxy
2019-10-08T20:54:11.9720184Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65222/merge:refs/remotes/pull/65222/merge
---
2019-10-08T21:00:19.5937819Z    Compiling cmake v0.1.38
2019-10-08T21:00:22.2408069Z error: variable does not need to be mutable
2019-10-08T21:00:22.2408429Z     --> src/libcore/iter/traits/iterator.rs:1731:39
2019-10-08T21:00:22.2408661Z      |
2019-10-08T21:00:22.2409001Z 1731 |     fn try_fold_self<F, R>(&mut self, mut f: F) -> Option<R>
2019-10-08T21:00:22.2409699Z      |                                       |
2019-10-08T21:00:22.2410015Z      |                                       help: remove this `mut`
2019-10-08T21:00:22.2410252Z      |
2019-10-08T21:00:22.2410559Z      = note: `-D unused-mut` implied by `-D warnings`
2019-10-08T21:00:22.2410559Z      = note: `-D unused-mut` implied by `-D warnings`
2019-10-08T21:00:22.2410600Z 
2019-10-08T21:00:22.2457110Z error: variable does not need to be mutable
2019-10-08T21:00:22.2457429Z     --> src/libcore/iter/traits/iterator.rs:1888:31
2019-10-08T21:00:22.2457689Z      |
2019-10-08T21:00:22.2457997Z 1888 |     fn fold_self<F>(mut self, mut f: F) -> Option<Self::Item>
2019-10-08T21:00:22.2459014Z      |                               |
2019-10-08T21:00:22.2459320Z      |                               help: remove this `mut`
2019-10-08T21:00:22.2459360Z 
2019-10-08T21:00:22.5164048Z    Compiling compiler_builtins v0.1.18
---
2019-10-08T21:00:25.0346879Z == clock drift check ==
2019-10-08T21:00:25.0363935Z   local time: Tue Oct  8 21:00:25 UTC 2019
2019-10-08T21:00:25.1350783Z   network time: Tue, 08 Oct 2019 21:00:25 GMT
2019-10-08T21:00:25.1351843Z == end clock drift check ==
2019-10-08T21:00:37.4745601Z ##[error]Bash exited with code '1'.
2019-10-08T21:00:37.4787879Z ##[section]Starting: Checkout
2019-10-08T21:00:37.4790296Z ==============================================================================
2019-10-08T21:00:37.4790352Z Task         : Get sources
2019-10-08T21:00:37.4790401Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
