
duplicate artfacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
the following dependencies are duplicated although they have the same features enabled:
  backtrace 0.3.9 (registry+https://github.com/rust-lang/crates.io-index)
    `clippy-driver` ("C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libbacktrace-6b9a1d9a5e670f0c.rlib")
    `cargo` ("C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libbacktrace-c21f6d630b07fdee.rlib")
the following dependencies have different features:
  winapi 0.3.5 (registry+https://github.com/rust-lang/crates.io-index)
    `clippy-driver` additionally enabled features {} at "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwinapi-69be4c44afc2c6f9.rlib"
    `cargo` additionally enabled features {"shellapi", "userenv"} at "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwinapi-31820f86a1bf3e6c.rlib"
