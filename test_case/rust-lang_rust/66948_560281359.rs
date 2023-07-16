plain
2019-12-02T07:47:09.9697307Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-02T07:47:09.9712034Z ##[command]git config gc.auto 0
2019-12-02T07:47:09.9717127Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-02T07:47:09.9720965Z ##[command]git config --get-all http.proxy
2019-12-02T07:47:09.9724707Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66948/merge:refs/remotes/pull/66948/merge
---
2019-12-02T08:09:07.7751708Z    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-12-02T08:09:10.0696082Z error: lifetime may not live long enough
2019-12-02T08:09:10.0696473Z    --> src/librustc_codegen_ssa/mir/mod.rs:177:25
2019-12-02T08:09:10.0696680Z     |
2019-12-02T08:09:10.0696957Z 122 | pub fn codegen_mir<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>>(
2019-12-02T08:09:10.0697329Z     |                    --  ---- lifetime `'tcx` defined here
2019-12-02T08:09:10.0697870Z     |                    lifetime `'a` defined here
2019-12-02T08:09:10.0698098Z ...
2019-12-02T08:09:10.0698098Z ...
2019-12-02T08:09:10.0698390Z 177 |     let memory_locals = analyze::non_ssa_locals(&fx);
2019-12-02T08:09:10.0698726Z     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'tcx` must outlive `'a`
2019-12-02T08:09:11.0872874Z error: aborting due to previous error
2019-12-02T08:09:11.0873042Z 
2019-12-02T08:09:11.1187436Z error: could not compile `rustc_codegen_ssa`.
2019-12-02T08:09:11.1187762Z warning: build failed, waiting for other jobs to finish...
---
2019-12-02T08:09:28.2195335Z   local time: Mon Dec  2 08:09:28 UTC 2019
2019-12-02T08:09:28.3014159Z   network time: Mon, 02 Dec 2019 08:09:28 GMT
2019-12-02T08:09:28.3020403Z == end clock drift check ==
2019-12-02T08:09:29.5444942Z 
2019-12-02T08:09:29.5500654Z ##[error]Bash exited with code '1'.
2019-12-02T08:09:29.5555436Z ##[section]Starting: Checkout
2019-12-02T08:09:29.5557500Z ==============================================================================
2019-12-02T08:09:29.5557713Z Task         : Get sources
2019-12-02T08:09:29.5557757Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
