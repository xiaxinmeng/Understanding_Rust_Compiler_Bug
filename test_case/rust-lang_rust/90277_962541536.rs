
$ rg 'during code'
rustc_codegen_gcc/src/coverageinfo.rs
59:64:/// `function_coverage_map` (keyed by function `Instance`) during codegen.

rustc_codegen_llvm/src/context.rs
65:49:/// when we ptrcast, and we have to ptrcast during codegen

rustc_codegen_llvm/src/coverageinfo/mod.rs
85:64:/// `function_coverage_map` (keyed by function `Instance`) during codegen.

rustc_codegen_ssa/src/back/write.rs
333:48:// Handler to use for diagnostics produced during codegen.
1824:29:bug!("panic during codegen/LLVM phase");

rustc_codegen_ssa/src/base.rs
715:51:// Since the main thread is sometimes blocked during codegen, we keep track

rustc_codegen_ssa/src/coverageinfo/map.rs
102:62:/// from `u32::MAX`. Theses operands are translated only during code generation, after all

rustc_codegen_ssa/src/traits/mod.rs
8:81://! actual codegen, while the builder stores the information about the function during codegen and

rustc_middle/src/middle/codegen_fn_attrs.rs
90:13:/// during codegen.

rustc_middle/src/mir/coverage.rs
48:9:/// during codegen. LLVM expects zero-based indexes.

rustc_middle/src/mir/mod.rs
1210:9:/// during codegen where distinct kinds of basic blocks may be

rustc_middle/src/traits/mod.rs
666:43:/// is `Obligation`, as one might expect. During codegen, however, this

rustc_middle/src/ty/instance.rs
347:46:/// in a monomorphic context (i.e., like during codegen), then it is guaranteed to return

rustc_middle/src/ty/mod.rs
1215:54:/// Typically, this is `Reveal::UserFacing`, but during codegen we
1314:30:/// the desired behavior during codegen and certain other special

rustc_middle/src/ty/sty.rs
1409:60:/// Erased region, used by trait selection, in MIR and during codegen.

rustc_middle/src/ty/util.rs
191:9:/// during codegen.
267:9:/// during codegen.

rustc_mir_transform/src/coverage/mod.rs
47:85:/// counters, via intrinsic `llvm.instrprof.increment`, and/or inject metadata used during codegen

rustc_mir_transform/src/coverage/query.rs
74:70:// allocate arrays when generating the coverage map (during codegen), so choose

rustc_mir_transform/src/lower_intrinsics.rs
93:28:// during codegen. Issue #35310.

rustc_mir_transform/src/remove_storage_markers.rs
1:64://! This pass removes storage markers if they won't be emitted during codegen.

rustc_trait_selection/src/opaque_types.rs
229:20:// during codegen.

rustc_trait_selection/src/traits/codegen.rs
57:65:"encountered ambiguity selecting `{:?}` during codegen, presuming due to \
70:77:"Encountered error `Unimplemented` selecting `{:?}` during codegen",
77:65:bug!("Encountered error `{:?}` selecting `{:?}` during codegen", e, trait_ref)
