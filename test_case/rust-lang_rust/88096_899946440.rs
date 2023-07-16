plain
   Compiling tracing-serde v0.1.2
   Compiling rls-span v0.5.3
   Compiling gsgdt v0.1.2
   Compiling rls-data v0.19.1
error: this trait bound has already been specified
   --> compiler/rustc_ast/src/visit.rs:355:8
355 |     V: Visitor<'a>,
    |        ^^^^^^^^^^^
    |
    |
    = note: `-D duplicate-bounds` implied by `-D warnings`
    = help: consider removing this trait bound
error: could not compile `rustc_ast` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:05:19
