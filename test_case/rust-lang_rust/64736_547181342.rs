plain
2019-10-28T22:36:57.8841386Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T22:36:57.9038378Z ##[command]git config gc.auto 0
2019-10-28T22:36:57.9118028Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T22:36:57.9192432Z ##[command]git config --get-all http.proxy
2019-10-28T22:36:57.9367154Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-10-28T22:53:38.2650143Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-28T22:53:39.8271193Z error: hidden lifetime parameters in types are deprecated
2019-10-28T22:53:39.8271758Z    --> src/librustc_mir/transform/mod.rs:303:22
2019-10-28T22:53:39.8271996Z     |
2019-10-28T22:53:39.8272388Z 303 | fn promoted_mir(tcx: TyCtxt, def_id: DefId) -> &IndexVec<Promoted, BodyCache> {
2019-10-28T22:53:39.8272781Z     |                      ^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-10-28T22:53:39.8273043Z     |
2019-10-28T22:53:39.8273339Z     = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
2019-10-28T22:53:39.8316931Z error: hidden lifetime parameters in types are deprecated
2019-10-28T22:53:39.8317276Z    --> src/librustc_mir/transform/mod.rs:303:68
2019-10-28T22:53:39.8317565Z     |
2019-10-28T22:53:39.8317565Z     |
2019-10-28T22:53:39.8317898Z 303 | fn promoted_mir(tcx: TyCtxt, def_id: DefId) -> &IndexVec<Promoted, BodyCache> {
2019-10-28T22:53:39.8318340Z     |                                                                    ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-10-28T22:53:50.1405396Z error: aborting due to 2 previous errors
2019-10-28T22:53:50.1410791Z 
2019-10-28T22:53:50.2347189Z error: could not compile `rustc_mir`.
2019-10-28T22:53:50.2385258Z warning: build failed, waiting for other jobs to finish...
---
2019-10-28T22:56:22.4418968Z   local time: Mon Oct 28 22:56:22 UTC 2019
2019-10-28T22:56:22.5956282Z   network time: Mon, 28 Oct 2019 22:56:22 GMT
2019-10-28T22:56:22.5957478Z == end clock drift check ==
2019-10-28T22:56:25.3782318Z 
2019-10-28T22:56:25.3894150Z ##[error]Bash exited with code '1'.
2019-10-28T22:56:25.3934969Z ##[section]Starting: Checkout
2019-10-28T22:56:25.3936866Z ==============================================================================
2019-10-28T22:56:25.3936928Z Task         : Get sources
2019-10-28T22:56:25.3936992Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
