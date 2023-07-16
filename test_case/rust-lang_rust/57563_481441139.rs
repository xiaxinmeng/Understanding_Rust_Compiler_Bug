rust
#[cfg(debug_assertions)] { A } // to get better debug-runtime perf
#[cfg(not(debug_assertions))] { Vec::new() } // to get the best release mode assembly
