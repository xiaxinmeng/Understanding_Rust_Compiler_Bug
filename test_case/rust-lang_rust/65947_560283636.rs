plain
2019-12-02T08:14:14.4537392Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-12-02T08:14:16.8790233Z error: lifetime may not live long enough
2019-12-02T08:14:16.8790659Z    --> src/librustc_codegen_ssa/mir/mod.rs:177:25
2019-12-02T08:14:16.8790914Z     |
2019-12-02T08:14:16.8791264Z 122 | pub fn codegen_mir<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>>(
2019-12-02T08:14:16.8791728Z     |                    --  ---- lifetime `'tcx` defined here
2019-12-02T08:14:16.8792410Z     |                    lifetime `'a` defined here
2019-12-02T08:14:16.8792666Z ...
2019-12-02T08:14:16.8792666Z ...
2019-12-02T08:14:16.8792981Z 177 |     let memory_locals = analyze::non_ssa_locals(&fx);
2019-12-02T08:14:16.8793419Z     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'tcx` must outlive `'a`
2019-12-02T08:14:17.7600084Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2019-12-02T08:14:18.0030312Z error: aborting due to previous error
2019-12-02T08:14:18.0030437Z 
2019-12-02T08:14:18.0339169Z error: could not compile `rustc_codegen_ssa`.
---
2019-12-02T08:14:18.6458199Z   local time: Mon Dec  2 08:14:18 UTC 2019
2019-12-02T08:14:18.9147771Z   network time: Mon, 02 Dec 2019 08:14:18 GMT
2019-12-02T08:14:18.9151756Z == end clock drift check ==
2019-12-02T08:14:20.3013295Z 
2019-12-02T08:14:20.3099685Z ##[error]Bash exited with code '1'.
2019-12-02T08:14:20.3127664Z ##[section]Starting: Checkout
2019-12-02T08:14:20.3129300Z ==============================================================================
2019-12-02T08:14:20.3129410Z Task         : Get sources
2019-12-02T08:14:20.3129483Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
