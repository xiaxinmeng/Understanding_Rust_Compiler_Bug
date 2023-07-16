plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0425]: cannot find value `extra_info` in this scope
    --> compiler/rustc_borrowck/src/region_infer/mod.rs:2214:46
     |
2214 |         debug!(?best_choice, ?blame_source, ?extra_info);

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_borrowck` due to previous error
warning: build failed, waiting for other jobs to finish...
