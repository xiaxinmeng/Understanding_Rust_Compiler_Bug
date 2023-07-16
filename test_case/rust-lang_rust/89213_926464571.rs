plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused variable: `memory_index_outer`
    --> compiler/rustc_middle/src/ty/layout.rs:1637:21
     |
1637 |                 let memory_index_outer = invert_mapping(&inverse_memory_index_outer);
     |                     ^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_memory_index_outer`
     |
     = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:08
