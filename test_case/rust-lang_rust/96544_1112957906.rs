plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0277]: `impl AsRef<Path>` cannot be sent between threads safely
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:2189:5
2189 |     join(
2189 |     join(
     |     ^^^^ `impl AsRef<Path>` cannot be sent between threads safely
2190 |         || encode_metadata_impl(tcx, path),
     |         ---------------------------------- within this `[closure@compiler/rustc_metadata/src/rmeta/encoder.rs:2190:9: 2190:43]`
     |
     = note: required because it appears within the type `[closure@compiler/rustc_metadata/src/rmeta/encoder.rs:2190:9: 2190:43]`
note: required by a bound in `rustc_data_structures::sync::join`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.2/src/join/mod.rs:97:25
     |
97   |     A: FnOnce() -> RA + Send,
     |                         ^^^^ required by this bound in `rustc_data_structures::sync::join`
     |
     |
2182 | pub fn encode_metadata(tcx: TyCtxt<'_>, path: impl AsRef<Path> + std::marker::Send) {

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_metadata` due to previous error
warning: build failed, waiting for other jobs to finish...
