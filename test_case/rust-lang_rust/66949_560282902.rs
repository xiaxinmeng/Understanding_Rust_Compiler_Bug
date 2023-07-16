plain
2019-12-02T07:48:32.7523148Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-02T07:48:32.7540182Z ##[command]git config gc.auto 0
2019-12-02T07:48:32.7546848Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-02T07:48:32.7604457Z ##[command]git config --get-all http.proxy
2019-12-02T07:48:32.7607233Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66949/merge:refs/remotes/pull/66949/merge
---
2019-12-02T08:13:44.6227728Z    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-12-02T08:13:47.3343212Z error: lifetime may not live long enough
2019-12-02T08:13:47.3343710Z    --> src/librustc_codegen_ssa/mir/mod.rs:177:25
2019-12-02T08:13:47.3344446Z     |
2019-12-02T08:13:47.3344857Z 122 | pub fn codegen_mir<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>>(
2019-12-02T08:13:47.3345300Z     |                    --  ---- lifetime `'tcx` defined here
2019-12-02T08:13:47.3345968Z     |                    lifetime `'a` defined here
2019-12-02T08:13:47.3346206Z ...
2019-12-02T08:13:47.3346206Z ...
2019-12-02T08:13:47.3346556Z 177 |     let memory_locals = analyze::non_ssa_locals(&fx);
2019-12-02T08:13:47.3351149Z     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'tcx` must outlive `'a`
2019-12-02T08:13:48.5685136Z error: aborting due to previous error
2019-12-02T08:13:48.5691711Z 
2019-12-02T08:13:48.6163181Z error: could not compile `rustc_codegen_ssa`.
2019-12-02T08:13:48.6182982Z warning: build failed, waiting for other jobs to finish...
---
2019-12-02T08:14:08.1782343Z   local time: Mon Dec  2 08:14:08 UTC 2019
2019-12-02T08:14:08.1782388Z   network time: Mon, 02 Dec 2019 08:14:08 GMT
2019-12-02T08:14:08.1782456Z == end clock drift check ==
2019-12-02T08:14:08.9104238Z 
2019-12-02T08:14:08.9207036Z ##[error]Bash exited with code '1'.
2019-12-02T08:14:08.9255414Z ##[section]Starting: Checkout
2019-12-02T08:14:08.9257137Z ==============================================================================
2019-12-02T08:14:08.9257206Z Task         : Get sources
2019-12-02T08:14:08.9257250Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
