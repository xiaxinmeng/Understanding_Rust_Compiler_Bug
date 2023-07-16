plain
2019-09-28T14:48:52.3840422Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-28T14:48:52.4004230Z ##[command]git config gc.auto 0
2019-09-28T14:48:52.4074224Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-28T14:48:52.4134507Z ##[command]git config --get-all http.proxy
2019-09-28T14:48:52.4264941Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-09-28T14:58:07.7080313Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-09-28T14:58:08.4099143Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-09-28T14:58:10.0053410Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-09-28T14:58:11.0805537Z error[E0412]: cannot find type `Body` in this scope
2019-09-28T14:58:11.0807249Z  --> src/librustc_mir/transform/ensure_predecessors_cache.rs:7:79
2019-09-28T14:58:11.0807499Z   |
2019-09-28T14:58:11.0808222Z 7 |     fn run_pass(&self, tcx: TyCtxt<'tcx>, source: MirSource<'tcx>, body: &mut Body<'tcx>) {
2019-09-28T14:58:11.0809129Z help: possible candidates are found in other modules, you can import them into scope
2019-09-28T14:58:11.0809398Z   |
2019-09-28T14:58:11.0809668Z 1 | use rustc::hir::Body;
2019-09-28T14:58:11.0809909Z   |
---
2019-09-28T14:58:20.7433303Z == clock drift check ==
2019-09-28T14:58:20.7450229Z   local time: Sat Sep 28 14:58:20 UTC 2019
2019-09-28T14:58:20.8330202Z   network time: Sat, 28 Sep 2019 14:58:20 GMT
2019-09-28T14:58:20.8330420Z == end clock drift check ==
2019-09-28T14:58:22.0584778Z ##[error]Bash exited with code '1'.
2019-09-28T14:58:22.0624129Z ##[section]Starting: Checkout
2019-09-28T14:58:22.0626097Z ==============================================================================
2019-09-28T14:58:22.0626165Z Task         : Get sources
2019-09-28T14:58:22.0626210Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
