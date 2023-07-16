plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no variant or associated item named `TargetPath` found for enum `rustc_target::spec::TargetTriple` in the current scope
    |
    |
368 |         TargetTriple::TargetPath(path) => {
    |                       ^^^^^^^^^^ variant or associated item not found in `rustc_target::spec::TargetTriple`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:54
