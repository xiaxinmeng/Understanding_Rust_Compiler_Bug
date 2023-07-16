plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VM1tupraLrZb
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
running: "xcrun" "--show-sdk-path" "--sdk" "iphoneos"
exit status: 0
Skipping Set({"src/doc"}) because it is excluded
Set({"src/librustc"}) not skipped for "bootstrap::dist::RustcDocs" -- not in ["src/doc", "extended"]
Set({}) not skipped for "bootstrap::dist::Mingw" -- not in ["src/doc", "extended"]
Set({"library/std"}) not skipped for "bootstrap::dist::Std" -- not in ["src/doc", "extended"]
Set({"library/std"}) not skipped for "bootstrap::dist::Std" -- not in ["src/doc", "extended"]
Set({"rustc-dev"}) not skipped for "bootstrap::dist::RustcDev" -- not in ["src/doc", "extended"]
Set({"analysis"}) not skipped for "bootstrap::dist::Analysis" -- not in ["src/doc", "extended"]
Set({"src"}) not skipped for "bootstrap::dist::Src" -- not in ["src/doc", "extended"]
Set({"cargo"}) not skipped for "bootstrap::dist::Cargo" -- not in ["src/doc", "extended"]
Set({"rls"}) not skipped for "bootstrap::dist::Rls" -- not in ["src/doc", "extended"]
Set({"rust-analyzer"}) not skipped for "bootstrap::dist::RustAnalyzer" -- not in ["src/doc", "extended"]
Set({"rust-demangler"}) not skipped for "bootstrap::dist::RustDemangler" -- not in ["src/doc", "extended"]
Set({"clippy"}) not skipped for "bootstrap::dist::Clippy" -- not in ["src/doc", "extended"]
Set({"miri"}) not skipped for "bootstrap::dist::Miri" -- not in ["src/doc", "extended"]
Set({"llvm-tools"}) not skipped for "bootstrap::dist::LlvmTools" -- not in ["src/doc", "extended"]
Set({"llvm-tools"}) not skipped for "bootstrap::dist::LlvmTools" -- not in ["src/doc", "extended"]
[TIMING] RustcDocs { host: TargetSelection { triple: "x86_64-apple-darwin", file: None } } -- 0.000
Set({"rust-dev"}) not skipped for "bootstrap::dist::RustDev" -- not in ["src/doc", "extended"]
Skipping Set({"extended"}) because it is excluded
[TIMING] RustcDocs { host: TargetSelection { triple: "x86_64-apple-ios", file: None } } -- 0.000
Set({"reproducible"}) not skipped for "bootstrap::dist::ReproducibleArtifacts" -- not in ["src/doc", "extended"]
[TIMING] RustcDocs { host: TargetSelection { triple: "aarch64-apple-ios-sim", file: None } } -- 0.000
[TIMING] RustcDocs { host: TargetSelection { triple: "aarch64-apple-ios-sim", file: None } } -- 0.000
Skipping Set({"src/doc"}) because it is excluded
[TIMING] Mingw { host: TargetSelection { triple: "x86_64-apple-darwin", file: None } } -- 0.000
Set({"src/librustc"}) not skipped for "bootstrap::dist::RustcDocs" -- not in ["src/doc", "extended"]
[TIMING] Mingw { host: TargetSelection { triple: "aarch64-apple-ios", file: None } } -- 0.000
Set({}) not skipped for "bootstrap::dist::Mingw" -- not in ["src/doc", "extended"]
Set({"src/librustc"}) not skipped for "bootstrap::dist::Rustc" -- not in ["src/doc", "extended"]
[TIMING] Mingw { host: TargetSelection { triple: "aarch64-apple-ios-sim", file: None } } -- 0.000
[TIMING] Assemble { target_compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } } } -- 0.000
[TIMING] StartupObjects { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None } } -- 0.000
---
warning: /Applications/Xcode_13.0.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ranlib: file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-94f045299be57459/out/libprofiler-rt.a(InstrProfilingPlatformFuchsia.o) has no symbols
warning: /Applications/Xcode_13.0.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ranlib: file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-94f045299be57459/out/libprofiler-rt.a(InstrProfilingPlatformLinux.o) has no symbols
warning: /Applications/Xcode_13.0.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ranlib: file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-94f045299be57459/out/libprofiler-rt.a(InstrProfilingPlatformOther.o) has no symbols
warning: /Applications/Xcode_13.0.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ranlib: file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-94f045299be57459/out/libprofiler-rt.a(InstrProfilingPlatformWindows.o) has no symbols
error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:113:19
113 |         pub size: Option<unsafe extern "C" fn(
    |                   ^^^^^^ not found in this scope
    |
help: consider importing this enum
help: consider importing this enum
    |
178 |         use Option;
    |

error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:117:21
117 |         pub malloc: Option<unsafe extern "C" fn(
    |                     ^^^^^^ not found in this scope
    |
help: consider importing this enum
help: consider importing this enum
    |
178 |         use Option;
    |

error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:121:21
121 |         pub calloc: Option<unsafe extern "C" fn(
    |                     ^^^^^^ not found in this scope
    |
help: consider importing this enum
help: consider importing this enum
    |
178 |         use Option;
    |

error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:126:21
    |
126 |         pub valloc: Option<unsafe extern "C" fn(
    |
help: consider importing this enum
    |
178 |         use Option;
178 |         use Option;
    |

error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:130:19
130 |         pub free: Option<unsafe extern "C" fn(
    |                   ^^^^^^ not found in this scope
    |
help: consider importing this enum
help: consider importing this enum
    |
178 |         use Option;
    |

error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:134:22
    |
134 |         pub realloc: Option<unsafe extern "C" fn(
    |
help: consider importing this enum
    |
178 |         use Option;
178 |         use Option;
    |

error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:139:22
    |
139 |         pub destroy: Option<unsafe extern "C" fn(zone: *mut malloc_zone_t)>,
    |
help: consider importing this enum
    |
178 |         use Option;
178 |         use Option;
    |

error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:141:27
    |
141 |         pub batch_malloc: Option<unsafe extern "C" fn(
    |
help: consider importing this enum
    |
178 |         use Option;
178 |         use Option;
    |

error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:147:25
    |
147 |         pub batch_free: Option<unsafe extern "C" fn(
    |
help: consider importing this enum
    |
178 |         use Option;
178 |         use Option;
    |

error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:154:23
154 |         pub memalign: Option<unsafe extern "C" fn(
    |                       ^^^^^^ not found in this scope
    |
help: consider importing this enum
help: consider importing this enum
    |
178 |         use Option;
    |

error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:159:33
159 |         pub free_definite_size: Option<unsafe extern "C" fn(
    |                                 ^^^^^^ not found in this scope
    |
help: consider importing this enum
help: consider importing this enum
    |
178 |         use Option;
    |

error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:164:30
    |
164 |         pub pressure_relief: Option<unsafe extern "C" fn(
    |
help: consider importing this enum
    |
178 |         use Option;
178 |         use Option;
    |

error[E0412]: cannot find type `Option` in this scope
   --> /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.105/src/unix/bsd/apple/b64/x86_64/mod.rs:168:30
168 |         pub claimed_address: Option<unsafe extern "C" fn(
    |                              ^^^^^^ not found in this scope
    |
help: consider importing this enum
