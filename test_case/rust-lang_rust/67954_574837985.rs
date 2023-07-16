plain
2020-01-15T20:14:48.6399702Z    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-01-15T20:15:43.7458688Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-01-15T20:15:50.6691734Z    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2020-01-15T20:16:18.7701513Z    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-01-15T20:17:02.4777694Z error[E0599]: no method named `generic_pass` found for type `rustc_data_structures::profiling::SelfProfilerRef` in the current scope
2020-01-15T20:17:02.4778331Z    --> src/librustc_codegen_llvm/back/lto.rs:596:23
2020-01-15T20:17:02.4778856Z     |
2020-01-15T20:17:02.4779244Z 596 |             cgcx.prof.generic_pass("LTO passes").run(|| {
2020-01-15T20:17:02.4779754Z     |                       ^^^^^^^^^^^^ method not found in `rustc_data_structures::profiling::SelfProfilerRef`
2020-01-15T20:17:04.2035443Z error: aborting due to previous error
2020-01-15T20:17:04.2035625Z 
2020-01-15T20:17:04.2035986Z For more information about this error, try `rustc --explain E0599`.
2020-01-15T20:17:04.2151734Z error: could not compile `rustc_codegen_llvm`.
---
2020-01-15T20:17:10.0132924Z   local time: Wed Jan 15 20:17:10 UTC 2020
2020-01-15T20:17:10.3648473Z   network time: Wed, 15 Jan 2020 20:17:10 GMT
2020-01-15T20:17:10.3652116Z == end clock drift check ==
2020-01-15T20:17:10.8343149Z 
2020-01-15T20:17:10.8522238Z ##[error]Bash exited with code '1'.
2020-01-15T20:17:10.8618303Z ##[section]Starting: Checkout
2020-01-15T20:17:10.8620471Z ==============================================================================
2020-01-15T20:17:10.8620581Z Task         : Get sources
2020-01-15T20:17:10.8620665Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
