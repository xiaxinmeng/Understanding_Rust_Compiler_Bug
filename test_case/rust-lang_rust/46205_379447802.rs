rust
src/librustc_lint/lib.rs
268:            id: LintId::of(INCOHERENT_FUNDAMENTAL_IMPLS),

src/librustc_typeck/coherence/inherent_impls_overlap.rs
52:                            lint::builtin::INCOHERENT_FUNDAMENTAL_IMPLS,

src/librustc/lint/builtin.rs
303:            INCOHERENT_FUNDAMENTAL_IMPLS,

src/test/compile-fail/issue-43355.rs
11:#![deny(incoherent_fundamental_impls)]

src/librustc/traits/specialize/mod.rs
351:                        lint::builtin::INCOHERENT_FUNDAMENTAL_IMPLS,
