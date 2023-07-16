plain
    Checking chalk-solve v0.55.0
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error: expected one of `(`, `.`, `::`, `=>`, `?`, or an operator, found `{`
     |
     |
2024 |         (Some(SymbolMangling::Legacy), _) if !debugging_opts.unstable_options {
     |                                                                               ^ expected one of `(`, `.`, `::`, `=>`, `?`, or an operator
    Checking chalk-engine v0.55.0
error: unreachable statement
    --> compiler/rustc_session/src/config.rs:2042:5
     |
     |
2023 | /     match (cg.symbol_mangling_version, debugging_opts.symbol_mangling_version) {
2024 | |         (Some(SymbolMangling::Legacy), _) if !debugging_opts.unstable_options {
2025 | |             early_error(
...    |
2039 | |         _ => {}
2040 | |     }
     | |_____- any code following this expression is unreachable
     | |_____- any code following this expression is unreachable
2041 | 
2042 | /     if debugging_opts.instrument_coverage.is_some()
2043 | |         && debugging_opts.instrument_coverage != Some(InstrumentCoverage::Off)
2044 | |     {
2045 | |         if cg.profile_generate.enabled() || cg.profile_use.is_some() {
2067 | |         }
2068 | |     }
     | |_____^ unreachable statement
     |
     |
     = note: `-D unreachable-code` implied by `-D warnings`
error: could not compile `rustc_session` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:01:59
