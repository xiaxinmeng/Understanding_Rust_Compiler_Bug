plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
    --> compiler/rustc_middle/src/ty/context.rs:2950:9
     |
2950 |         tcx.arena.alloc(tcx.resolutions(()).glob_map.get(&id).cloned().unwrap_or_default())
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_data_structures::stable_set::StableSet`, found struct `HashSet`
     = note:      expected reference `&rustc_data_structures::stable_set::StableSet<rustc_span::Symbol>`
     = note:      expected reference `&rustc_data_structures::stable_set::StableSet<rustc_span::Symbol>`
             found mutable reference `&mut HashSet<rustc_span::Symbol, BuildHasherDefault<FxHasher>>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
