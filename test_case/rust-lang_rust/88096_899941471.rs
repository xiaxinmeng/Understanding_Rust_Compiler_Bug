plain
   Compiling chalk-ir v0.55.0
   Compiling tracing v0.1.25
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error: this trait bound has already been specified
   |
   |
80 |         U: Clone + Debug + Eq + Hash + Clone,
   |
   |
   = note: `-D duplicate-bounds` implied by `-D warnings`
   = help: consider removing this trait bound
error: could not compile `rustc_data_structures` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:06:46
