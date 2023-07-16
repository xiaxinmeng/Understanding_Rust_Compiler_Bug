plain
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
   Compiling rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error: prefer `FxHashMap` over `HashMap`, it has better performance
   --> compiler/rustc_mir_transform/src/boxy_thing.rs:205:24
    |
205 |     otherwise_refines: HashMap<BasicBlock, Option<RefineRange>>,
    |
    |
    = note: `-D rustc::default-hash-types` implied by `-D warnings`
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
error: could not compile `rustc_mir_transform` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:10:26
