plain
[RUSTC-TIMING] rustc_workspace_hack test:false 0.059
   Compiling rls v2.0.0 (C:\a\rust\rust\src\tools\rls)
[RUSTC-TIMING] rls test:false 1.573
    Finished release [optimized] target(s) in 1m 23s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
the following dependencies have different features:
the following dependencies have different features:
  windows-sys 0.45.0 (registry+https://github.com/rust-lang/crates.io-index)
    `rls` additionally enabled features {} at "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwindows_sys-10be35956adfb82f.rlib"
    `cargo` additionally enabled features {"Win32_System_SystemServices", "Win32_System_IO", "Win32_System_Threading", "Win32_Security", "Win32_System_JobObjects"} at "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwindows_sys-017053a858ddbe8e.rlib"
  windows-sys 0.42.0 (registry+https://github.com/rust-lang/crates.io-index)
    `rls` additionally enabled features {} at "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwindows_sys-4a646961a669c101.rlib"
    `cargo` additionally enabled features {"Win32_System_WindowsProgramming", "Win32_System_Pipes", "Win32_System_SystemServices", "Win32_System_LibraryLoader"} at "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwindows_sys-f50abfd148aec17b.rlib"
  winapi 0.3.9 (registry+https://github.com/rust-lang/crates.io-index)
    `rls` additionally enabled features {} at "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwinapi-ee48548943c65f3e.rlib"
    `cargo` additionally enabled features {"std", "fileapi", "winerror"} at "C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwinapi-e5d0ce59fd6ee153.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', tool.rs:250:13
Build completed unsuccessfully in 0:49:46
