plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: expected expression, found `?`
   --> compiler/rustc_codegen_ssa/src/back/link.rs:848:17
    |
848 |                 ?cmd, %out,
    |                 ^ expected expression

error: expected `,`, found `?`
   --> compiler/rustc_codegen_ssa/src/back/link.rs:857:17
    |
856 |                  tried to call it, automatically retrying again."
    |                                                                  - expected `,`
857 |                 ?cmd, %out, status = %output.status,

    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: could not compile `rustc_codegen_ssa` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
