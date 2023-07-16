
Building stage1 std artifacts (x86_64-pc-windows-gnu -> x86_64-pc-windows-gnu)
running: "C:\\msys64\\home\\we\\rust\\build\\x86_64-pc-windows-gnu\\stage0\\bin\\cargo.exe" "build" "-Zconfig-profile" "--target" "x86_64-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "8" "-v" "-v" "-v" "-v" "-v" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "C:\\msys64\\home\\we\\rust\\src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint] stale: changed "C:\\msys64\\home\\we\\rust\\build\\x86_64-pc-windows-gnu\\stage1-std\\x86_64-pc-windows-gnu\\release\\deps\\libcore-2b12e3ff20349daa.rlib"
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint]           (vs) "C:\\msys64\\home\\we\\rust\\build\\x86_64-pc-windows-gnu\\stage1-std\\x86_64-pc-windows-gnu\\release\\.fingerprint\\backtrace-f79483b0a3bcfa52\\dep-lib-backtrace-f79483b0a3bcfa52"
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint]                FileTime { seconds: 13223331039, nanos: 670850900 } != FileTime { seconds: 13223331039, nanos: 689869600 }
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint] fingerprint error for test v0.0.0 (C:\msys64\home\we\rust\src\libtest)/Build/Target { ..: lib_target("test", ["dylib", "rlib"], "C:\\msys64\\home\\we\\rust\\src\\libtest\\lib.rs", Edition2018) }
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint]     err: current filesystem status shows we're outdated
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint] fingerprint error for getopts v0.2.21/Build/Target { ..: lib_target("getopts", ["lib"], "C:\\Users\\we\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\getopts-0.2.21\\src\\lib.rs", Edition2015) }
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint]     err: current filesystem status shows we're outdated
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint] fingerprint error for rustc-std-workspace-std v1.99.0 (C:\msys64\home\we\rust\src\tools\rustc-std-workspace-std)/Build/Target { ..: lib_target("rustc-std-workspace-std", ["lib"], "C:\\msys64\\home\\we\\rust\\src\\tools\\rustc-std-workspace-std\\lib.rs", Edition2018) }
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint]     err: current filesystem status shows we're outdated
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint] fingerprint error for std v0.0.0 (C:\msys64\home\we\rust\src\libstd)/Build/Target { ..: lib_target("std", ["dylib", "rlib"], "C:\\msys64\\home\\we\\rust\\src\\libstd\\lib.rs", Edition2018) }
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint]     err: current filesystem status shows we're outdated
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint] fingerprint error for backtrace v0.3.40/Build/Target { ..: lib_target("backtrace", ["lib"], "C:\\Users\\we\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\backtrace-0.3.40\\src\\lib.rs", Edition2018) }
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint]     err: current filesystem status shows we're outdated
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint] fingerprint error for unicode-width v0.1.6/Build/Target { ..: lib_target("unicode-width", ["lib"], "C:\\Users\\we\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\unicode-width-0.1.6\\src\\lib.rs", Edition2015) }
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint]     err: current filesystem status shows we're outdated
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint] fingerprint error for proc_macro v0.0.0 (C:\msys64\home\we\rust\src\libproc_macro)/Build/Target { ..: lib_target("proc_macro", ["lib"], "C:\\msys64\\home\\we\\rust\\src\\libproc_macro\\lib.rs", Edition2018) }
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint]     err: current filesystem status shows we're outdated
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint] fingerprint error for term v0.0.0 (C:\msys64\home\we\rust\src\libterm)/Build/Target { ..: lib_target("term", ["lib"], "C:\\msys64\\home\\we\\rust\\src\\libterm\\lib.rs", Edition2018) }
[2020-01-12T19:31:02Z INFO  cargo::core::compiler::fingerprint]     err: current filesystem status shows we're outdated
       Fresh cc v1.0.49
       Fresh core v0.0.0 (C:\msys64\home\we\rust\src\libcore)
       Fresh autocfg v0.1.6
       Fresh rustc-std-workspace-core v1.99.0 (C:\msys64\home\we\rust\src\tools\rustc-std-workspace-core)
       Fresh compiler_builtins v0.1.22
       Fresh libc v0.2.66
       Fresh cfg-if v0.1.8
       Fresh alloc v0.0.0 (C:\msys64\home\we\rust\src\liballoc)
       Fresh rustc-demangle v0.1.16
       Fresh backtrace-sys v0.1.32
       Fresh panic_abort v0.0.0 (C:\msys64\home\we\rust\src\libpanic_abort)
       Fresh rustc-std-workspace-alloc v1.99.0 (C:\msys64\home\we\rust\src\tools\rustc-std-workspace-alloc)
       Fresh unwind v0.0.0 (C:\msys64\home\we\rust\src\libunwind)
   Compiling backtrace v0.3.40
       Fresh hashbrown v0.6.2
       Fresh panic_unwind v0.0.0 (C:\msys64\home\we\rust\src\libpanic_unwind)
