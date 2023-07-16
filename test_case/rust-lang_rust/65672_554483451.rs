plain
2019-11-15T18:33:21.7769582Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-15T18:33:21.7957723Z ##[command]git config gc.auto 0
2019-11-15T18:33:21.8029097Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-15T18:33:21.8082668Z ##[command]git config --get-all http.proxy
2019-11-15T18:33:21.8231038Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65672/merge:refs/remotes/pull/65672/merge
---
2019-11-15T18:44:29.6775412Z    Compiling syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-11-15T18:45:35.9147402Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-11-15T18:49:07.7662333Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-11-15T18:49:44.5945583Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-11-15T18:49:45.8129452Z error[E0425]: cannot find value `dead_unwinds` in this scope
2019-11-15T18:49:45.8129820Z    --> src/librustc_mir/dataflow/generic/engine.rs:138:18
2019-11-15T18:49:45.8130421Z     |
2019-11-15T18:49:45.8130663Z 138 |                 &dead_unwinds,
2019-11-15T18:49:45.8131005Z     |                  ^^^^^^^^^^^^ help: you might have meant to use the available field: `self.dead_unwinds`
2019-11-15T18:49:45.8177868Z error[E0425]: cannot find value `dead_unwinds` in this scope
2019-11-15T18:49:45.8177868Z error[E0425]: cannot find value `dead_unwinds` in this scope
2019-11-15T18:49:45.8178249Z    --> src/librustc_mir/dataflow/generic/engine.rs:146:18
2019-11-15T18:49:45.8178502Z     |
2019-11-15T18:49:45.8178746Z 146 |                 &dead_unwinds,
2019-11-15T18:49:45.8179084Z     |                  ^^^^^^^^^^^^ help: you might have meant to use the available field: `self.dead_unwinds`
2019-11-15T18:49:45.8179152Z 
2019-11-15T18:49:46.5974069Z error[E0687]: lifetimes used in `fn` or `Fn` syntax must be explicitly declared using `<...>` binders
2019-11-15T18:49:46.5974446Z    --> src/librustc_mir/dataflow/generic/engine.rs:194:87
2019-11-15T18:49:46.5974777Z     |
2019-11-15T18:49:46.5975302Z 194 |     apply_block_effect: impl Fn(&mut BitSet<A::Idx>, BasicBlock, &mir::BasicBlockData<'tcx>),
2019-11-15T18:49:46.5975728Z     |                                                                                       ^^^^ in-band lifetime definition
2019-11-15T18:49:46.7384273Z error: aborting due to 3 previous errors
2019-11-15T18:49:46.7384367Z 
2019-11-15T18:49:46.7389179Z For more information about this error, try `rustc --explain E0425`.
2019-11-15T18:49:46.7967745Z error: could not compile `rustc_mir`.
---
2019-11-15T18:51:38.1849393Z   local time: Fri Nov 15 18:51:38 UTC 2019
2019-11-15T18:51:38.4638204Z   network time: Fri, 15 Nov 2019 18:51:38 GMT
2019-11-15T18:51:38.4639013Z == end clock drift check ==
2019-11-15T18:51:41.2185168Z 
2019-11-15T18:51:41.2285902Z ##[error]Bash exited with code '1'.
2019-11-15T18:51:41.2318316Z ##[section]Starting: Checkout
2019-11-15T18:51:41.2319915Z ==============================================================================
2019-11-15T18:51:41.2319980Z Task         : Get sources
2019-11-15T18:51:41.2320036Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
