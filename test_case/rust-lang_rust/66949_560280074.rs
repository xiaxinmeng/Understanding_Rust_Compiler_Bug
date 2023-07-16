plain
2019-12-02T07:59:00.0867876Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-12-02T07:59:02.6071671Z error: lifetime may not live long enough
2019-12-02T07:59:02.6072268Z    --> src/librustc_codegen_ssa/mir/mod.rs:177:25
2019-12-02T07:59:02.6072595Z     |
2019-12-02T07:59:02.6073006Z 122 | pub fn codegen_mir<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>>(
2019-12-02T07:59:02.6073486Z     |                    --  ---- lifetime `'tcx` defined here
2019-12-02T07:59:02.6074231Z     |                    lifetime `'a` defined here
2019-12-02T07:59:02.6074501Z ...
2019-12-02T07:59:02.6074501Z ...
2019-12-02T07:59:02.6074862Z 177 |     let memory_locals = analyze::non_ssa_locals(&fx);
2019-12-02T07:59:02.6075328Z     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'tcx` must outlive `'a`
2019-12-02T07:59:03.7333326Z error: aborting due to previous error
2019-12-02T07:59:03.7338050Z 
2019-12-02T07:59:03.7641912Z error: could not compile `rustc_codegen_ssa`.
2019-12-02T07:59:03.7664012Z warning: build failed, waiting for other jobs to finish...
---
2019-12-02T07:59:10.0540152Z   local time: Mon Dec  2 07:59:10 UTC 2019
2019-12-02T07:59:10.2049540Z   network time: Mon, 02 Dec 2019 07:59:10 GMT
2019-12-02T07:59:10.2054236Z == end clock drift check ==
2019-12-02T07:59:11.5846390Z 
2019-12-02T07:59:11.5946719Z ##[error]Bash exited with code '1'.
2019-12-02T07:59:11.5978027Z ##[section]Starting: Checkout
2019-12-02T07:59:11.5979962Z ==============================================================================
2019-12-02T07:59:11.5980077Z Task         : Get sources
2019-12-02T07:59:11.5980164Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
