plain
   Compiling rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0412]: cannot find type `SmallVec` in this scope
   --> compiler/rustc_session/src/filesearch.rs:125:32
    |
125 | pub fn sysroot_candidates() -> SmallVec<[PathBuf; 2]> {

For more information about this error, try `rustc --explain E0412`.
error: could not compile `rustc_session` due to previous error
warning: build failed, waiting for other jobs to finish...
