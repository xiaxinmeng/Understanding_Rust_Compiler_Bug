plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: unreachable pattern
    --> compiler/rustc_trait_selection/src/traits/select/mod.rs:1741:55
     |
1741 |             (AutoImplCandidate(_), ImplCandidate(_) | ProjectionCandidate(_))
     |
     |
     = note: `-D unreachable-patterns` implied by `-D warnings`
error: unreachable pattern
    --> compiler/rustc_trait_selection/src/traits/select/mod.rs:1742:35
     |
     |
1742 |             | (ImplCandidate(_) | ProjectionCandidate(_), AutoImplCandidate(_)) => false,

error: could not compile `rustc_trait_selection` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
