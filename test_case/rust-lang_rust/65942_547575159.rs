plain
2019-10-29T18:27:51.8910920Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-29T18:27:51.9132076Z ##[command]git config gc.auto 0
2019-10-29T18:27:51.9193870Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-29T18:27:51.9254254Z ##[command]git config --get-all http.proxy
2019-10-29T18:27:51.9400893Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65942/merge:refs/remotes/pull/65942/merge
---
2019-10-29T18:47:34.4166477Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-29T18:47:35.6672163Z error[E0425]: cannot find function `collect_temps_and_candidates` in this scope
2019-10-29T18:47:35.6672658Z     --> src/librustc_mir/transform/promote_consts.rs:1198:22
2019-10-29T18:47:35.6672952Z      |
2019-10-29T18:47:35.6674174Z 1198 |     let (temps, _) = collect_temps_and_candidates(tcx, body, &mut rpo);
2019-10-29T18:47:35.6674608Z      |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `collect_temps_and_valid_candidates`
2019-10-29T18:47:41.5899547Z error[E0308]: mismatched types
2019-10-29T18:47:41.5899961Z     --> src/librustc_mir/transform/promote_consts.rs:1200:15
2019-10-29T18:47:41.5900260Z      |
2019-10-29T18:47:41.5900260Z      |
2019-10-29T18:47:41.5900832Z 1200 |         item: Item::new(tcx, mir_def_id, body),
2019-10-29T18:47:41.5901966Z      |               |
2019-10-29T18:47:41.5902402Z      |               expected reference, found struct `transform::check_consts::Item`
2019-10-29T18:47:41.5902402Z      |               expected reference, found struct `transform::check_consts::Item`
2019-10-29T18:47:41.5902798Z      |               help: consider borrowing here: `&Item::new(tcx, mir_def_id, body)`
2019-10-29T18:47:41.5903087Z      |
2019-10-29T18:47:41.5903436Z      = note: expected type `&transform::check_consts::Item<'_, '_>`
2019-10-29T18:47:41.5903774Z                 found type `transform::check_consts::Item<'_, '_>`
2019-10-29T18:47:45.8725207Z error: aborting due to 2 previous errors
2019-10-29T18:47:45.8725908Z 
2019-10-29T18:47:45.8726589Z Some errors have detailed explanations: E0308, E0425.
2019-10-29T18:47:45.8726928Z For more information about an error, try `rustc --explain E0308`.
---
2019-10-29T18:48:56.8905592Z   local time: Tue Oct 29 18:48:56 UTC 2019
2019-10-29T18:48:56.8905645Z   network time: Tue, 29 Oct 2019 18:48:56 GMT
2019-10-29T18:48:56.8905696Z == end clock drift check ==
2019-10-29T18:48:59.1575080Z 
2019-10-29T18:48:59.1688628Z ##[error]Bash exited with code '1'.
2019-10-29T18:48:59.1724341Z ##[section]Starting: Checkout
2019-10-29T18:48:59.1725905Z ==============================================================================
2019-10-29T18:48:59.1725955Z Task         : Get sources
2019-10-29T18:48:59.1726012Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
