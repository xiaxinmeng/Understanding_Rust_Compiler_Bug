
duplicate artfacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
the following dependencies are duplicated although they have the same features enabled:
  backtrace 0.3.9 (registry+https://github.com/rust-lang/crates.io-index)
    `clippy-driver` ("C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage2-tools\\x86_64-pc-windows-gnu\\release\\deps\\libbacktrace-7dc4ef752c85274e.rlib")
    `cargo` ("C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage2-tools\\x86_64-pc-windows-gnu\\release\\deps\\libbacktrace-ab50bab4d87e7bbe.rlib")
the following dependencies have different features:
  winapi 0.3.5 (registry+https://github.com/rust-lang/crates.io-index)
    `clippy-driver` additionally enabled features {} at "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage2-tools\\x86_64-pc-windows-gnu\\release\\deps\\libwinapi-b494f3abb913a03b.rlib"
    `cargo` additionally enabled features {"userenv"} at "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage2-tools\\x86_64-pc-windows-gnu\\release\\deps\\libwinapi-e648ff91915e2746.rlib"
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', 
