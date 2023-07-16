
    Finished release [optimized] target(s) in 3m 59s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:

the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  winapi 0.3.8 (registry+https://github.com/rust-lang/crates.io-index)
    `rls` additionally enabled features {"aclapi", "accctrl"} at "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage1-tools\\x86_64-pc-windows-gnu\\release\\deps\\libwinapi-721dad5408882873.rlib"
    `cargo` additionally enabled features {} at "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage1-tools\\x86_64-pc-windows-gnu\\release\\deps\\libwinapi-049f94892a54a8d5.rlib"
